use bcrypt::{hash, verify};
use log::debug;
use serde_json::json;
use warp::{http::header::*, hyper::StatusCode, reject::Rejection, reply::Reply};

use crate::{
    error::{convert_to_rejection, Error},
    jwt::generate_token,
};
use db::{
    db_pool::{DbConnection, DieselError, PooledPgConnection},
    functions::user::{create_user_record, delete_user_record, query_user_info},
    structs::{User, UserRole},
};

pub async fn create_user(db_conn: DbConnection, user: User) -> Result<impl Reply, Rejection> {
    let conn: PooledPgConnection = db_conn.map_err(convert_to_rejection)?;

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
            create_user_record(conn, &user).map_err(convert_to_rejection)?;

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

pub async fn delete_user(db_conn: DbConnection, user: User) -> Result<impl Reply, Rejection> {
    let conn: PooledPgConnection = db_conn.map_err(convert_to_rejection)?;

    // running query
    delete_user_record(conn, &user).map_err(convert_to_rejection)?;

    Ok(warp::reply::json(&json!({
        "msg": format!("user {} deleted", user.user_name)
    })))
}

pub async fn get_user_name(db_conn: DbConnection, user_id: i32) -> Result<impl Reply, Rejection> {
    let conn: PooledPgConnection = db_conn.map_err(convert_to_rejection)?;
    let mut id = User::new();
    id.set_id(user_id);

    // running query
    let query = query_user_info(conn, &id).map_err(convert_to_rejection)?;

    Ok(warp::reply::json(&json!({
        "msg": query.user_name
    })))
}

pub async fn login_user_route(db_conn: DbConnection, user: User) -> Result<impl Reply, Rejection> {
    let conn: PooledPgConnection = db_conn.map_err(convert_to_rejection)?;

    let query = query_user_info(conn, &user).map_err(convert_to_rejection)?;

    if verify(&user.user_pwd, &query.user_pwd).map_err(convert_to_rejection)? {
        let token = generate_token(query).map_err(convert_to_rejection)?;
        let cookie = format!(
            // the below jwt works in dev server + HTTP (lack of Secure flag)
            "jwt={}; Path=/; HttpOnly; Max-Age=1209600; SameSite=Strict;Secure",
            token
        );
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

async fn encrypt_pwd(pwd: &str) -> Result<String, Rejection> {
    Ok(hash(pwd, 4).map_err(convert_to_rejection)?)
}
