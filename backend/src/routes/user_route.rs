use bcrypt::hash;
use serde_json::json;
use warp::{reject::Rejection, reply::Reply};

use crate::error::{convert_to_rejection, Error};
use db::{
    db_pool::{DbConnection, PooledPgConnection},
    functions::user::{create_user_record, delete_user_record, get_user_name},
    structs::User,
};

pub async fn create_user(db_conn: DbConnection, user: User) -> Result<impl Reply, Rejection> {
    let conn: PooledPgConnection = db_conn.map_err(convert_to_rejection)?;

    match user.validate(&user.user_pwd) {
        Ok(_) => {
            let user = User {
                id: user.id,
                user_name: user.user_name,
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

pub async fn get_user_info(db_conn: DbConnection, user_id: i32) -> Result<impl Reply, Rejection> {
    let conn: PooledPgConnection = db_conn.map_err(convert_to_rejection)?;
    // running query
    let usr = get_user_name(conn, user_id).map_err(convert_to_rejection)?;

    Ok(warp::reply::json(&json!({
        "msg": usr
    })))
}

async fn encrypt_pwd(pwd: &str) -> Result<String, Rejection> {
    Ok(hash(pwd, 4).map_err(convert_to_rejection)?)
}
