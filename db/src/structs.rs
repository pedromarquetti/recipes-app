#[cfg(not(target_arch = "wasm32"))]
use crate::schema::{recipe, recipe_ingredient, recipe_step, recipe_users};
use std::{
    fmt::Debug,
    io::{Error as IOError, Write},
};

use diesel::{
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    prelude::*,
    serialize::{IsNull, ToSql},
    sql_types::Text,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// This struct represents possible Ok Values the API can generate
pub struct ApiOkResponse {
    pub msg: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(
        Queryable,
        Selectable,
        AsChangeset,
        Identifiable,
        Associations,
        Insertable,
    )
)]
#[cfg_attr(not(target_arch = "wasm32"), 
    diesel(belongs_to(Recipe)),
    diesel(table_name = recipe_step),
)]
pub struct Step {
    pub id: Option<i32>,
    pub recipe_id: i32,
    pub step_name: String,
    pub step_instruction: String,
    pub step_duration_min: i32,
}
impl Default for Step {
    fn default() -> Self {
        Step {
            id: None,
            recipe_id: 0,
            step_name: String::new(),
            step_instruction: String::new(),
            step_duration_min: 0,
        }
    }
}
impl Step {
    pub fn set_recipe_id(&mut self, id: i32) {
        self.recipe_id = id
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(
        Queryable,
        Selectable,
        AsChangeset,
        Identifiable,
        Associations,
        Insertable
    )
)]
#[cfg_attr(not(target_arch="wasm32"), 
    diesel(belongs_to(Recipe)),
    diesel(table_name = recipe_ingredient),
)]
pub struct Ingredient {
    pub id: Option<i32>,
    pub recipe_id: i32,
    pub ingredient_name: String,
    pub ingredient_quantity: i32,
    pub quantity_unit: String,
}
impl Default for Ingredient {
    fn default() -> Self {
        Ingredient {
            id: None,
            recipe_id: 0,
            ingredient_name: String::new(),
            ingredient_quantity: 0,
            quantity_unit: String::new(),
        }
    }
}
impl Ingredient {
    pub fn set_recipe_id(&mut self, id: i32) {
        self.recipe_id = id
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// configuring attributes
// if target_arch (architeture the code is being compiled in) is wasm32, ignore these (diesel stuff)
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(
        AsChangeset,
        Queryable,
        Selectable,
        Associations,
        Insertable,
        Identifiable,
    )
)]
#[cfg_attr(not(target_arch = "wasm32"), 
    diesel(belongs_to(User)),
    diesel(table_name = recipe),
)]
pub struct Recipe {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub recipe_name: String,
    pub recipe_observations: Option<Vec<String>>,
}
impl Default for Recipe {
    fn default() -> Self {
        Recipe {
            id: None,
            user_id: None,
            recipe_name: String::new(),
            recipe_observations: None,
        }
    }
}
impl Recipe {
    /// Change recipe name
    pub fn set_name<S>(&mut self, name: S)
    where
        S: Into<String>,
    {
        self.recipe_name = name.into();
    }
    pub fn set_id(&mut self, recipe_id: i32) {
        self.id = Some(recipe_id)
    }
    pub fn set_user_id(&mut self, user_id: i32) {
        self.user_id = Some(user_id)
    }

    pub fn set_observation(&mut self, obs: Option<Vec<String>>) {
        self.recipe_observations = obs
    }
}

pub trait RecipeTrait {}
impl RecipeTrait for FullRecipe {}
impl RecipeTrait for Recipe {}
impl RecipeTrait for Ingredient {}
impl RecipeTrait for Step {}
impl<T> RecipeTrait for Vec<T> {}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct UrlRecipeQuery {
    pub id: Option<i32>,
    pub name: Option<String>,
}

impl Default for UrlRecipeQuery {
    fn default() -> Self {
        UrlRecipeQuery {
            id: None,
            name: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Queryable))]
/// used to represent a recipe with its steps and ingredients
pub struct FullRecipe {
    pub recipe: Recipe,
    pub ingredients: Vec<Ingredient>,
    pub steps: Vec<Step>,
    pub recipe_owner_name: String,
}
impl Default for FullRecipe {
    fn default() -> Self {
        FullRecipe {
            recipe: Recipe::default(),
            ingredients: vec![],
            steps: vec![],
            recipe_owner_name: String::new(),
        }
    }
}
impl FullRecipe {
    /// creates new FullRecipe with Recipe id
    pub fn new_with_id(id: i32) -> Self {
        Self {
            recipe: Recipe {
                id: Some(id),
                ..Default::default()
            },
            ..Default::default()
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
    pub fn set_owner_name(&mut self, name: String) {
        self.recipe_owner_name = name
    }
    /// replaces item from list (ingredient)
    pub fn replace_steps(&mut self, replace_item: Step) -> Result<Vec<Step>, String> {
        let mut input_vec = self.steps.clone();
        let idx = input_vec.iter().position(|item| {
            // matching item id with replace item id
            item.id.eq(&replace_item.id)
        });
        match idx {
            Some(idx) => {
                input_vec[idx] = replace_item;
                Ok(input_vec)
            }
            None => return Err(String::from("could not find index")),
        }
    }
    /// replaces item from list (ingredient)
    ///
    /// Returns an error as String if fails
    pub fn replace_ingredient(
        &mut self,
        replace_item: Ingredient,
    ) -> Result<Vec<Ingredient>, String> {
        let mut input_vec = self.ingredients.clone();
        let idx = input_vec.iter().position(|ingredient| {
            // matching item id with replace item id
            ingredient.id.eq(&replace_item.id)
        });
        match idx {
            Some(idx) => {
                input_vec[idx] = replace_item;
                Ok(input_vec)
            }
            None => return Err(String::from("could not find index")),
        }
    }
}

#[derive(Deserialize, Clone, Debug, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(FromSqlRow, AsExpression))]
#[cfg_attr(not(target_arch = "wasm32"), diesel(sql_type = Text))
]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
}

#[cfg(not(target_arch = "wasm32"))]
impl ToSql<Text, Pg> for UserRole {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match self {
            UserRole::User => {
                // self.to_sql(out)
                out.write(b"user")?
            }
            UserRole::Admin => out.write(b"admin")?,
        };
        Ok(IsNull::No)
    }
}
#[cfg(not(target_arch = "wasm32"))]

impl FromSql<Text, Pg> for UserRole {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"user" => Ok(UserRole::User),
            b"admin" => Ok(UserRole::Admin),
            x => Err(format!("unknown variant: {:?},", x).into()),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[cfg_attr(not(target_arch="wasm32"), 
    // derive( Queryable, Selectable, Insertable, Identifiable),
    derive(
        AsChangeset,
        Queryable,
        Selectable,
        Insertable,
        Identifiable,
    ),
)]
#[cfg_attr(not(target_arch="wasm32"), 
    diesel(table_name = recipe_users)
)]
pub struct User {
    pub id: Option<i32>,
    pub user_name: String,
    pub user_role: UserRole,
    pub user_pwd: String,
}
impl Default for User {
    fn default() -> Self {
        User {
            id: None,
            user_name: String::new(),
            user_role: UserRole::User,
            user_pwd: String::new(),
        }
    }
}

impl User {
    /// modify Steps inside FullRecipe
    pub fn set_id(&mut self, user_id: i32) {
        self.id = Some(user_id)
    }
    /// Password validation function
    pub fn validate(&self, pwd: &str) -> Result<String, IOError> {
        return Ok(pwd.into());
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct UrlUserQuery {
    pub id: Option<i32>,
    pub name: Option<String>,
}

impl Default for UrlUserQuery {
    fn default() -> Self {
        UrlUserQuery {
            id: None,
            name: None,
        }
    }
}
