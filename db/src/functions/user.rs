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
