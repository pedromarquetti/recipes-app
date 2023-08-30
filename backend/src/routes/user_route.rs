use diesel::{ExpressionMethods, RunQueryDsl};
use serde_json::json;
use warp::{reject::Rejection, reply::Reply};

use crate::{
    db::{recipe::User, DbConnection, OkPool},
    error::convert_to_rejection,
};

pub async fn create_user(db_conn: DbConnection, usr: User) -> Result<impl Reply, Rejection> {
    let mut conn: OkPool = db_conn.map_err(convert_to_rejection)?;
    use crate::schema::recipe_users;

    diesel::insert_into(recipe_users::table)
        .values::<&User>(&usr)
        .execute(&mut conn)
        .map_err(convert_to_rejection)?;

    Ok(warp::reply::json(&json!({
        "msg": format!("user {} created", usr.user_name)
    })))
}
pub async fn delete_user(db_conn: DbConnection, usr: User) -> Result<impl Reply, Rejection> {
    let mut conn = db_conn.map_err(convert_to_rejection)?;
    use crate::schema::recipe_users;

    diesel::delete(recipe_users::table)
        .filter(recipe_users::user_name.eq(&usr.user_name))
        .execute(&mut conn)
        .map_err(convert_to_rejection)?;

    Ok(warp::reply::json(&json!({
        "msg": format!("user {} created", usr.user_name)
    })))
}
