use bcrypt::{hash, verify};
use serde_json::json;
use warp::{reject::Rejection, reply::Reply};

use crate::error::{convert_to_rejection, Error};
use db::{
    db_pool::{DbConnection, DieselError, PooledPgConnection},
    functions::user::{create_user_record, delete_user_record, query_user_info},
    structs::User,
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
    let usr = query_user_info(conn, &id).map_err(convert_to_rejection)?;

    Ok(warp::reply::json(&json!({
        "msg": usr.user_name
    })))
}
pub async fn login_user(db_conn: DbConnection, user: User) -> Result<impl Reply, Rejection> {
    let conn: PooledPgConnection = db_conn.map_err(convert_to_rejection)?;
    let pwd_query = query_user_info(conn, &user);
    match pwd_query {
        Ok(pwd) => {
            if verify(&user.user_pwd, &pwd.user_pwd).map_err(convert_to_rejection)? {
                Ok(warp::reply::json(&json!(
                    {
                        "msg":format!("user {} logged in!",user.user_name)
                    }
                )))
            } else {
                Ok(warp::reply::json(&json!(
                    {
                        "msg":""
                    }
                )))
            }
        }
        Err(DieselError::NotFound) => Err(Error::db_error("user not found").into()),
        Err(err) => Err(Error::db_error(format!("An error occurred! {err}")).into()),
    }
}

async fn encrypt_pwd(pwd: &str) -> Result<String, Rejection> {
    Ok(hash(pwd, 4).map_err(convert_to_rejection)?)
}
