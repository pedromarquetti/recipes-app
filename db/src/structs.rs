use std::io::Error;

use crate::schema::{recipe, recipe_ingredient, recipe_step, recipe_users};

use diesel::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(not(target_arch="wasm32"), 
    derive(Queryable,
    Selectable,AsChangeset,
    Identifiable,
    Associations,
    Insertable,),
    diesel(table_name = recipe_step),
    diesel(belongs_to(Recipe))
)]
pub struct Step {
    pub id: Option<i32>,
    pub recipe_id: i32,
    pub step_name: String,
    pub step_instruction: String,
    pub step_duration_min: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(not(target_arch="wasm32"), 
    derive(Queryable,
    Selectable,AsChangeset,
    Identifiable,
    Associations,
    Insertable,),
    diesel(table_name = recipe_ingredient),
    diesel(belongs_to(Recipe))
)]
pub struct Ingredient {
    pub id: Option<i32>,
    pub recipe_id: i32,
    pub ingredient_name: String,
    pub ingredient_quantity: i32,
    pub quantity_unit: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// configuring attributes
// if target_arch (architeture the code is being compiled in) is wasm32, ignore these (diesel stuff)
#[cfg_attr(not(target_arch = "wasm32"), 
    derive(AsChangeset,Queryable,Selectable,Associations,Insertable,Identifiable,),
    diesel(belongs_to(User)),
    diesel(table_name = recipe),
)]
pub struct Recipe {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub recipe_name: String,
    pub recipe_observations: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Queryable))]
/// used to represent a recipe with its steps
pub struct FullRecipe {
    pub recipe: Recipe,
    pub ingredients: Vec<Ingredient>,
    pub steps: Vec<Step>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[cfg_attr(not(target_arch="wasm32"), 
    derive( Queryable, Selectable, Insertable, Identifiable),
    diesel(table_name = recipe_users)
)]

pub struct User {
    pub id: Option<i32>,
    pub user_name: String,
    pub user_pwd: String,
}
impl User {
    pub fn validate(&self, pwd: &str) -> Result<String, Error> {
        return Ok(pwd.into());
    }
}
