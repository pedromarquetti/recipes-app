use crate::schema::recipe_users::dsl as user_dsl;
use diesel::prelude::*;

use crate::{
    db_pool::{DieselError, PooledPgConnection},
    structs::User,
};

pub fn create_user_record(mut conn: PooledPgConnection, user: &User) -> Result<(), DieselError> {
    use crate::schema::recipe_users;
    diesel::insert_into(recipe_users::table)
        .values::<&User>(&user)
        .execute(&mut conn)?;
    Ok(())
}
pub fn delete_user_record(mut conn: PooledPgConnection, user: &User) -> Result<(), DieselError> {
    use crate::schema::recipe_users;
    diesel::delete(recipe_users::table)
        .filter(recipe_users::user_name.eq(&user.user_name))
        .execute(&mut conn)?;
    Ok(())
}

pub fn get_user_name(mut conn: PooledPgConnection, user_id: i32) -> Result<String, DieselError> {
    let user_name: String = user_dsl::recipe_users
        .filter(user_dsl::id.eq(user_id))
        .select(user_dsl::user_name)
        .first(&mut conn)?;
    Ok(user_name)
}
