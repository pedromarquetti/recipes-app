use serde_json::json;
use warp::{reject::Rejection, reply::Reply};

use crate::error::convert_to_rejection;
use db::{
    db_pool::{DbConnection, PooledPgConnection},
    functions::user::{create_user_record, delete_user_record},
    structs::User,
};

pub async fn create_user(db_conn: DbConnection, user: User) -> Result<impl Reply, Rejection> {
    let conn: PooledPgConnection = db_conn.map_err(convert_to_rejection)?;

    // running query
    create_user_record(conn, &user).map_err(convert_to_rejection)?;

    Ok(warp::reply::json(&json!({
        "msg": format!("user {} created", user.user_name)
    })))
}
pub async fn delete_user(db_conn: DbConnection, user: User) -> Result<impl Reply, Rejection> {
    let conn = db_conn.map_err(convert_to_rejection)?;

    // running query
    delete_user_record(conn, &user).map_err(convert_to_rejection)?;

    Ok(warp::reply::json(&json!({
        "msg": format!("user {} deleted", user.user_name)
    })))
}
