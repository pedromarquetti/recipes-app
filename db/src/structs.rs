use std::io::Error;

use crate::schema::{recipe, recipe_ingredient, recipe_step, recipe_users};

use diesel::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// This struct represents possible Ok Values the API can generate
pub struct ApiOkResponse {
    pub msg: String,
}

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
impl Step {
    pub fn new() -> Self {
        Step {
            id: None,
            recipe_id: 0,
            step_name: String::new(),
            step_instruction: String::new(),
            step_duration_min: 0,
        }
    }
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
impl Ingredient {
    /// Empty Full Recipe, meant to be used as a placeholder
    pub fn new() -> Self {
        Ingredient {
            id: None,
            recipe_id: 0,
            ingredient_name: String::new(),
            ingredient_quantity: 0,
            quantity_unit: String::new(),
        }
    }
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
impl Recipe {
    pub fn new() -> Self {
        Recipe {
            id: None,
            user_id: None,
            recipe_name: String::new(),
            recipe_observations: None,
        }
    }

    /// Change recipe name
    pub fn set_name<S>(&mut self, name: S)
    where
        S: Into<String>,
    {
        self.recipe_name = name.into();
    }

    pub fn set_observation(&mut self, obs: Option<Vec<String>>) {
        self.recipe_observations = obs
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Queryable))]
/// used to represent a recipe with its steps and ingredients
pub struct FullRecipe {
    pub recipe: Recipe,
    pub ingredients: Vec<Ingredient>,
    pub steps: Vec<Step>,
}

impl FullRecipe {
    /// Empty Full Recipe, meant to be used as a placeholder
    pub fn new() -> Self {
        FullRecipe {
            recipe: Recipe::new(),
            steps: vec![Step::new()],
            ingredients: vec![Ingredient::new()],
        }
    }
    /// modify Recipe inside FullRecipe
    pub fn set_recipe(&mut self, recipe: Recipe) {
        self.recipe = recipe
    }
    /// modify Ingredients inside FullRecipe
    pub fn set_ingredients(&mut self, ingredient: Vec<Ingredient>) {
        self.ingredients = ingredient
    }
    /// modify Steps inside FullRecipe
    pub fn set_steps(&mut self, steps: Vec<Step>) {
        self.steps = steps
    }
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
    pub fn new() -> Self {
        User {
            id: None,
            user_name: "".into(),
            user_pwd: "".into(),
        }
    }
    pub fn validate(&self, pwd: &str) -> Result<String, Error> {
        return Ok(pwd.into());
    }
}

pub trait RecipeTrait {}
impl RecipeTrait for FullRecipe {}
impl RecipeTrait for Recipe {}
impl RecipeTrait for Ingredient {}
impl RecipeTrait for Step {}
