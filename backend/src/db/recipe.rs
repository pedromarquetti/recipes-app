use crate::schema::{recipe, recipe_step, recipe_users};

use diesel::prelude::*;

use serde::Deserialize;

#[derive(
    Deserialize, Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Insertable,
)]
#[diesel(table_name = recipe_step)]
#[diesel(belongs_to(Recipe))]
pub struct Step {
    pub id: Option<i32>,
    pub step_name: String,
    pub step_instruction: String,
    pub step_duration_min: i32,
    pub recipe_id: i32,
}

#[derive(
    Deserialize, Queryable, Selectable, Associations, Insertable, Identifiable, Debug, PartialEq,
)]
#[diesel(belongs_to(User))]
#[diesel(table_name = recipe)]
pub struct Recipe {
    pub id: Option<i32>,
    pub recipe_name: String,
    pub recipe_ingredients: Vec<String>,
    pub recipe_observations: Option<Vec<String>>,
    pub user_id: Option<i32>,
}

#[derive(Deserialize, Queryable, Selectable, Insertable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = recipe_users)]
pub struct User {
    pub id: Option<i32>,
    pub user_name: String,
    pub user_pwd: String,
}
