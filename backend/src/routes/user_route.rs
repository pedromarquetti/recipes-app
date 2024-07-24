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
    structs::{UrlUserQuery, User, UserRole},
};

pub async fn create_user(db_conn: DbConnection, user: User) -> Result<impl Reply, Rejection> {
    let mut conn: PooledPgConnection = db_conn.map_err(convert_to_rejection)?;

    match user.validate(&user.user_pwd) {
        Ok(_) => {
            let user = User {
                id: user.id,
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
    if check_user_permission(&usr, user_claims) {
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
    if check_user_permission(&query, user_claims) {
        return Ok(warp::reply::json(&json!({"msg": query.user_name})));
    }
    Err(Error::user_error("User cannot be viewed", StatusCode::FORBIDDEN).into())
}

pub async fn login_user_route(db_conn: DbConnection, user: User) -> Result<impl Reply, Rejection> {
    let mut conn: PooledPgConnection = db_conn.map_err(convert_to_rejection)?;

    let query = query_user_info(
        &mut conn,
        &UrlUserQuery {
            id: user.id,
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
    user: User,
    user_claims: Option<UserClaims>,
) -> Result<impl Reply, Rejection> {
    let mut conn: PooledPgConnection = db_conn.map_err(convert_to_rejection)?;
    let usr = query_user_info(
        &mut conn,
        &UrlUserQuery {
            id: user.id,
            name: Some(user.user_name.clone()),
        },
    )
    .map_err(convert_to_rejection)?;
    if check_user_permission(&usr, user_claims) {
        // running query
        update_user_record(&mut conn, &user).map_err(convert_to_rejection)?;

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
fn check_user_permission(user: &User, claims: Option<UserClaims>) -> bool {
    if claims.is_none()
        || user
            .id
            .expect("expected valid User ID")
            .ne(&claims.as_ref().expect("expected valid TOKEN").user_id)
    {
        // return false if no token found OR user id != claim
        return false;
    } else {
        if let Some(user_claims) = claims {
            // token found
            if user_claims.role.eq(&UserRole::Admin) {
                // admins can edit any user
                return true;
            }
        }
        return true;
    }
}
