use crate::{schema::recipe_users::dsl as user_dsl, structs::UrlUserQuery};
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

pub fn delete_user_record(
    mut conn: PooledPgConnection,
    user: &UrlUserQuery,
) -> Result<(), DieselError> {
    use crate::schema::recipe_users;
    diesel::delete(recipe_users::table)
        .filter(recipe_users::user_name.eq(&user.name.as_ref().unwrap()))
        .execute(&mut conn)?;
    Ok(())
}

pub fn query_user_info(
    conn: &mut PooledPgConnection,
    user: &UrlUserQuery,
) -> Result<User, DieselError> {
    if let Some(name) = user.name.as_ref() {
        return Ok(user_dsl::recipe_users
            .select((
                user_dsl::id,
                user_dsl::user_name,
                user_dsl::user_role,
                user_dsl::user_pwd,
            ))
            .filter(user_dsl::user_name.eq(&name))
            .first::<User>(conn)?);
    } else if let Some(id) = user.id {
        return Ok(user_dsl::recipe_users
            .select((
                user_dsl::id,
                user_dsl::user_name,
                user_dsl::user_role,
                user_dsl::user_pwd,
            ))
            .filter(user_dsl::id.eq(&id))
            .first::<User>(conn)?);
    }
    Err(DieselError::NotFound)
}

pub fn update_user_record(conn: &mut PooledPgConnection, user: &User) -> Result<(), DieselError> {
    diesel::update(user_dsl::recipe_users)
        .filter(user_dsl::id.eq(user.id))
        .set(user)
        .execute(conn)?;
    Ok(())
}

pub fn list_users_query(conn: &mut PooledPgConnection) -> Result<Vec<User>, DieselError> {
    let users: Vec<User> = user_dsl::recipe_users
        .select((
            user_dsl::id,
            user_dsl::user_name,
            user_dsl::user_role,
            user_dsl::user_pwd,
        ))
        .get_results(conn)?;
    Ok(users)
}
