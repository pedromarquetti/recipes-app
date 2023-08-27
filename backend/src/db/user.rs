use crate::schema::recipe_users;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Queryable, Insertable)]
#[diesel(table_name = recipe_users)]
pub struct Recipe {
    pub user_name: String,
    pub user_pwd: String,
}
