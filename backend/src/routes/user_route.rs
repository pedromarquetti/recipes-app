use crate::error::Error;
use bcrypt::{hash, verify};

use serde_json::json;
use warp::{http::header::*, hyper::StatusCode, reject::Rejection, reply::Reply};

use crate::{
    error::convert_to_rejection,
    is_dev_server,
    jwt::{generate_token, UserClaims},
};
use db::{
    db_pool::{DbConnection, PooledPgConnection},
    functions::user::{
        create_user_record, delete_user_record, list_users_query, query_user_info,
        update_user_record,
    },
    structs::{NewUser, UpdateUser, UrlUserQuery, User, UserRole},
};

pub async fn create_user(db_conn: DbConnection, user: NewUser) -> Result<impl Reply, Rejection> {
    let mut conn: PooledPgConnection = db_conn.map_err(convert_to_rejection)?;

    match user.validate(&user.user_pwd) {
        Ok(_) => {
            let user = NewUser {
                user_name: user.user_name,
                user_role: user.user_role,
                // encrypting password
                user_pwd: encrypt_pwd(&user.user_pwd).await?,
            };
            // running query
            create_user_record(&mut conn, &user).map_err(convert_to_rejection)?;

            Ok(warp::reply::json(&json!({
                "msg": format!("user {} created", user.user_name)
            })))
        }
        Err(err) => {
            return Err(Error::payload_error(format!(
                "invalid password {}",
                err.kind().to_string()
            ))
            .into())
        }
    }
}

pub async fn delete_user(
    user_query: UrlUserQuery,
    user_claims: Option<UserClaims>,
    db_conn: DbConnection,
) -> Result<impl Reply, Rejection> {
    let mut conn: PooledPgConnection = db_conn.map_err(convert_to_rejection)?;
    if user_query.id.is_none() {
        return Err(Error::payload_error("missing user ID field").into());
    }

    // getting user info inserted in URL
    let usr = query_user_info(&mut conn, &user_query).map_err(convert_to_rejection)?;

    // check if user can delete queried user (admins only) OR if user can delete themselves
    if check_user_permission(&usr, &user_claims) {
        // running query
        if delete_user_record(&mut conn, &user_query).map_err(convert_to_rejection)? == 0 {
            return Err(Error::not_found("User not found").into());
        }
        return Ok(warp::reply::json(&json!({
            "msg": format!("user deleted")
        })));
    }
    return Err(Error::user_error("User cannot be deleted", StatusCode::FORBIDDEN).into());
}

pub async fn get_user_name(
    user_id: UrlUserQuery,
    user_claims: Option<UserClaims>,
    db_conn: DbConnection,
) -> Result<impl Reply, Rejection> {
    if user_id.id.is_none() {
        return Err(Error::payload_error("missing user ID field (.../id=<user_id>)").into());
    }

    let mut conn: PooledPgConnection = db_conn.map_err(convert_to_rejection)?;

    // running query
    let query = query_user_info(&mut conn, &user_id).map_err(convert_to_rejection)?;
    if check_user_permission(&query, &user_claims) {
        return Ok(warp::reply::json(&json!({"msg": query.user_name})));
    }
    Err(Error::user_error("User cannot be viewed", StatusCode::FORBIDDEN).into())
}

pub async fn login_user_route(db_conn: DbConnection, user: User) -> Result<impl Reply, Rejection> {
    let mut conn: PooledPgConnection = db_conn.map_err(convert_to_rejection)?;

    let query = query_user_info(
        &mut conn,
        &UrlUserQuery {
            id: None,
            name: Some(user.user_name),
        },
    )
    .map_err(convert_to_rejection)?;

    if verify(&user.user_pwd, &query.user_pwd).map_err(convert_to_rejection)? {
        let token = generate_token(query).map_err(convert_to_rejection)?;
        let cookie: String;
        if is_dev_server() {
            // dev server uses http
            cookie = format!(
                // the below jwt works in dev server + HTTP (lack of Secure flag)
                "jwt={}; Path=/; HttpOnly; Max-Age=1209600; SameSite=Strict",
                token
            );
        } else {
            cookie = format!(
                // the below jwt works in prod server + HTTPS (Secure flag)
                "jwt={}; Path=/; HttpOnly; Max-Age=1209600; SameSite=Strict;Secure",
                token
            );
        }
        let json_resp = warp::reply::json(&json!(
            {
                "msg":format!("login success!",)
            }
        ));
        Ok(warp::reply::with_header(json_resp, SET_COOKIE, cookie))
    } else {
        Err(Error::user_error("Incorred User or Password!", StatusCode::UNAUTHORIZED).into())
    }
}

pub async fn update_user_info_route(
    db_conn: DbConnection,
    input_user: UpdateUser,
    user_claims: Option<UserClaims>,
) -> Result<impl Reply, Rejection> {
    if input_user.id < 0 {
        return Err(Error::user_error("invalid user id", StatusCode::UNAUTHORIZED).into());
    }
    // pwd validation
    if let Some(pwd) = &input_user.user_pwd {
        match NewUser::default().validate(pwd) {
            Err(err) => {
                return Err(Error::from(err).into());
            }
            _ => {}
        }
    }
    let mut conn: PooledPgConnection = db_conn.map_err(convert_to_rejection)?;
    let old_user = query_user_info(
        &mut conn,
        &UrlUserQuery {
            id: Some(input_user.id),
            name: None,
        },
    )
    .map_err(convert_to_rejection)?;
    if check_user_permission(&old_user, &user_claims) {
        let mut updated_user: User = Default::default();
        updated_user.set_id(old_user.id);

        // conditionally checking what fields the user wants to update
        if let Some(user_name) = input_user.user_name {
            updated_user.user_name = user_name
        } else {
            // if None, set updated == old part
            updated_user.user_name = old_user.user_name.clone()
        }
        if let Some(user_role) = input_user.user_role {
            if user_claims.unwrap().role == UserRole::Admin {
                updated_user.user_role = user_role
            }
        }
        if let Some(user_pwd) = input_user.user_pwd {
            updated_user.user_pwd = encrypt_pwd(&user_pwd).await?
        } else {
            updated_user.user_pwd = old_user.user_pwd.clone()
        }

        // running query
        update_user_record(&mut conn, &updated_user).map_err(convert_to_rejection)?;

        return Ok(warp::reply::json(&json!({
            "msg": format!("user updated")
        })));
    }
    return Err(Error::user_error("User cannot be updated", StatusCode::FORBIDDEN).into());
}

pub async fn list_users(
    db_conn: DbConnection,
    user_claims: Option<UserClaims>,
) -> Result<impl Reply, Rejection> {
    if user_claims.is_none()
        || user_claims
            .expect("expected valid token")
            .role
            .eq(&UserRole::User)
    {
        return Err(Error::user_error("Cannot see list of users", StatusCode::FORBIDDEN).into());
    }

    let mut conn: PooledPgConnection = db_conn.map_err(convert_to_rejection)?;

    let users = list_users_query(&mut conn).map_err(convert_to_rejection)?;

    return Ok(warp::reply::json(&json!({"msg":users})));
}

async fn encrypt_pwd(pwd: &str) -> Result<String, Rejection> {
    Ok(hash(pwd, 4).map_err(convert_to_rejection)?)
}

/// Function to check user permission
///
/// Admins can edit any user
///
/// Users can edit themselves
///
/// Returns true if user can edit
fn check_user_permission(user: &User, claims: &Option<UserClaims>) -> bool {
    if let Some(claim) = claims {
        // UserClaims found!
        if claim.role == UserRole::Admin || user.id.eq(&claim.user_id) {
            // return true if user is admin OR user id matches
            return true;
        }
    }
    false
}

#[cfg(test)]
mod user_permission_test {
    use db::structs::{User, UserRole};

    use crate::{jwt::UserClaims, routes::user_route::check_user_permission};

    #[test]
    fn test_check_user_perm() {
        let normal_user = User {
            id: 1,
            user_role: UserRole::User,
            ..Default::default()
        };
        let admin_user = User {
            id: 2,
            user_role: UserRole::Admin,
            ..Default::default()
        };
        let admin_claims = UserClaims {
            user_id: 2,
            role: UserRole::Admin,
            ..Default::default()
        };
        let normal_claims = UserClaims {
            user_id: 1,
            role: UserRole::User,
            ..Default::default()
        };
        let invalid_claims = User {
            id: -23,
            user_role: UserRole::Admin,
            ..Default::default()
        };
        // no claims, should return false
        assert!(check_user_permission(&normal_user, &None) == false);

        // normal user with matching normal claims, should return true
        assert!(check_user_permission(&normal_user, &Some(normal_claims.clone())) == true);

        // admin trying to access  normal user, should return true
        assert!(check_user_permission(&normal_user, &Some(admin_claims)) == true);

        // normal claims trying to access admin_user, should return false
        assert!(check_user_permission(&admin_user, &Some(normal_claims.clone())) == false);

        // invalid id with admin role, should return false
        assert!(check_user_permission(&invalid_claims, &Some(normal_claims)) == false)
    }
}
