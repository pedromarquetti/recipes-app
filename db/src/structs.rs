use crate::schema::{recipe, recipe_step, recipe_users};

use diesel::prelude::*;

use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[derive(
    Serialize,
    Deserialize,
    Queryable,
    Selectable,
    Identifiable,
    Associations,
    Debug,
    PartialEq,
    Insertable,
)]
#[diesel(table_name = recipe_step)]
#[diesel(belongs_to(Recipe))]
pub struct Step {
    pub id: Option<i32>,
    pub recipe_id: i32,
    pub step_name: String,
    pub step_instruction: String,
    pub step_duration_min: i32,
}

#[derive(
    Queryable,
    Serialize,
    Deserialize,
    Debug,
    Selectable,
    Associations,
    Insertable,
    Identifiable,
    PartialEq,
)]
#[diesel(belongs_to(User))]
#[diesel(table_name = recipe)]
pub struct Recipe {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub recipe_name: String,
    pub recipe_ingredients: Vec<String>,
    pub recipe_observations: Option<Vec<String>>,
}

#[derive(Deserialize, Queryable, Debug, PartialEq)]
/// used to represent a recipe with its steps
pub struct FullRecipe {
    pub recipe: Recipe,
    pub steps: Vec<Step>,
}

impl Serialize for FullRecipe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("FullRecipe", 2)?;
        s.serialize_field("recipe", &self.recipe)?;
        s.serialize_field("steps", &self.steps)?;
        s.end()
    }
}

#[derive(Deserialize, Queryable, Selectable, Insertable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = recipe_users)]
pub struct User {
    pub id: Option<i32>,
    pub user_name: String,
    pub user_pwd: String,
}
