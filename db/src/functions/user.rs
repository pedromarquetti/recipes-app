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

pub fn query_user_info(mut conn: PooledPgConnection, user: &User) -> Result<User, DieselError> {
    let result: User = user_dsl::recipe_users
        .select((
            user_dsl::id,
            user_dsl::user_name,
            user_dsl::user_role,
            user_dsl::user_pwd,
        ))
        .filter(user_dsl::user_name.eq(&user.user_name))
        .first::<User>(&mut conn)?;
    Ok(result)
}

pub fn update_user_record(mut conn: PooledPgConnection, user: &User) -> Result<(), DieselError> {
    diesel::update(user_dsl::recipe_users)
        .filter(user_dsl::id.eq(user.id))
        .set(user)
        .execute(&mut conn)?;
    Ok(())
}
