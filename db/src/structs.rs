#[cfg(not(target_arch = "wasm32"))]
use crate::schema::{recipe, recipe_ingredient, recipe_step, recipe_users};
use std::{
    fmt::{Debug, Display},
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

pub trait RecipeTrait {}
impl RecipeTrait for FullRecipe {}

impl RecipeTrait for NewRecipe {}
impl RecipeTrait for UpdateRecipe {}
impl RecipeTrait for Recipe {}

impl RecipeTrait for User {}
impl RecipeTrait for NewUser {}
impl RecipeTrait for UpdateUser {}

impl RecipeTrait for Ingredient {}
impl RecipeTrait for NewIngredient {}
impl RecipeTrait for UpdateIngredient {}

impl RecipeTrait for Step {}
impl RecipeTrait for NewStep {}
impl RecipeTrait for UpdateStep {}

impl<T> RecipeTrait for Vec<T> {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(Queryable, Selectable, AsChangeset, Identifiable, Associations,)
)]
#[cfg_attr(not(target_arch = "wasm32"), 
    diesel(belongs_to(Recipe)),
    diesel(table_name = recipe_step),
)]
pub struct Step {
    pub id: i32,
    pub recipe_id: i32,
    pub step_name: String,
    pub step_instruction: String,
    pub step_duration_min: i32,
}
impl Default for Step {
    fn default() -> Self {
        Step {
            id: -1,
            recipe_id: -1,
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

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct UpdateStep {
    pub id: i32,
    pub recipe_id: i32,
    pub step_name: Option<String>,
    pub step_instruction: Option<String>,
    pub step_duration_min: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(AsChangeset, Associations, Insertable,)
)]
#[cfg_attr(not(target_arch = "wasm32"), 
    diesel(belongs_to(Recipe)),
    diesel(table_name = recipe_step),
)]
pub struct NewStep {
    pub recipe_id: i32,
    pub step_name: String,
    pub step_instruction: String,
    pub step_duration_min: i32,
}
impl Default for NewStep {
    fn default() -> Self {
        Self {
            recipe_id: -1,
            step_name: String::new(),
            step_instruction: String::new(),
            step_duration_min: -1,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(Queryable, Selectable, AsChangeset, Identifiable, Associations,)
)]
#[cfg_attr(not(target_arch="wasm32"), 
    diesel(belongs_to(Recipe)),
    diesel(table_name = recipe_ingredient),
)]
pub struct Ingredient {
    pub id: i32,
    pub recipe_id: i32,
    pub ingredient_name: String,
    pub ingredient_quantity: i32,
    pub quantity_unit: String,
}
impl Default for Ingredient {
    fn default() -> Self {
        Ingredient {
            id: -1,
            recipe_id: -1,
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

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct UpdateIngredient {
    pub id: i32,
    pub recipe_id: i32,
    pub ingredient_name: Option<String>,
    pub ingredient_quantity: Option<i32>,
    pub quantity_unit: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(AsChangeset, Associations, Insertable)
)]
#[cfg_attr(not(target_arch="wasm32"), 
    diesel(belongs_to(Recipe)),
    diesel(table_name = recipe_ingredient),
)]
pub struct NewIngredient {
    pub recipe_id: i32,
    pub ingredient_name: String,
    pub ingredient_quantity: i32,
    pub quantity_unit: String,
}
impl Default for NewIngredient {
    fn default() -> Self {
        Self {
            recipe_id: -1,
            ingredient_name: String::new(),
            ingredient_quantity: -1,
            quantity_unit: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// configuring attributes
// if target_arch (architeture the code is being compiled in) is wasm32, ignore these (diesel stuff)
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(AsChangeset, Queryable, Selectable, Associations, Identifiable,)
)]
#[cfg_attr(not(target_arch = "wasm32"), 
    diesel(belongs_to(User)),
    diesel(table_name = recipe),
)]
pub struct Recipe {
    pub id: i32,
    pub user_id: i32,
    pub recipe_name: String,
    pub recipe_observations: Option<Vec<Option<String>>>,
}
impl Default for Recipe {
    fn default() -> Self {
        Recipe {
            id: -1,
            user_id: -1,
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
        self.id = recipe_id
    }
    pub fn set_user_id(&mut self, user_id: i32) {
        self.user_id = user_id
    }

    pub fn set_observation(&mut self, obs: Option<Vec<Option<String>>>) {
        self.recipe_observations = obs
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct UpdateRecipe {
    pub id: i32,
    pub user_id: i32,
    pub recipe_name: Option<String>,
    pub recipe_observations: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// configuring attributes
// if target_arch (architeture the code is being compiled in) is wasm32, ignore these (diesel stuff)
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(AsChangeset, Queryable, Selectable, Insertable)
)]
#[cfg_attr(not(target_arch = "wasm32"), 
    diesel(belongs_to(User)),
    diesel(table_name = recipe),
)]
pub struct NewRecipe {
    pub user_id: i32,
    pub recipe_name: String,
    pub recipe_observations: Option<Vec<Option<String>>>,
}
impl NewRecipe {
    pub fn set_user_id(&mut self, id: i32) {
        self.user_id = id;
    }
}
impl Default for NewRecipe {
    fn default() -> Self {
        Self {
            user_id: -1,
            recipe_name: String::new(),
            recipe_observations: None,
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

    /// Replaces item from list
    ///
    /// # Returns
    ///
    /// Returns an error as String if fails
    ///
    /// Returns a new Vec with modified items
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

    /// Replaces item from list
    ///
    /// # Returns
    ///
    /// Returns an error as String if fails
    ///
    /// Returns a new Vec with modified items
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

    /// Removes item from list
    ///
    /// # Returns
    ///
    /// Returns an error as String if fails
    ///
    /// Returns a new Vec with modified items
    pub fn remove_ingredient(&mut self, input_id: i32) -> Result<Vec<Ingredient>, String> {
        let mut tmp = self.ingredients.clone();
        let idx = tmp.iter().position(|ingredient: &Ingredient| {
            ingredient
                .id
                // if no id is present, unwrap and set it to -1 (invalid, will return "no idx found")
                .eq(&input_id)
        });
        match idx {
            Some(idx) => {
                tmp.remove(idx);
                return Ok(tmp);
            }
            None => return Err(String::from("No index found")),
        }
    }

    /// Removes item from list
    ///
    /// # Returns
    ///
    /// Returns an error as String if fails
    ///
    /// Returns a new Vec with modified items
    pub fn remove_step(&mut self, input_id: i32) -> Result<Vec<Step>, String> {
        let mut tmp = self.steps.clone();
        let idx = tmp
            .iter()
            .position(|ingredient: &Step| ingredient.id.eq(&input_id));
        match idx {
            Some(idx) => {
                tmp.remove(idx);
                return Ok(tmp);
            }
            None => return Err(String::from("No index found")),
        }
    }

    /// Step finder from ID
    pub fn get_step(&mut self, input_id: i32) -> Result<Step, String> {
        let tmp = self.steps.clone();
        let idx = self
            .steps
            .iter()
            .position(|step: &Step| step.id.eq(&input_id));
        match idx {
            Some(idx) => {
                return Ok(tmp[idx].clone());
            }
            None => return Err(String::from("No index found")),
        }
    }

    /// Ingredient finder from ID
    pub fn get_ingredient(&mut self, input_id: i32) -> Result<Ingredient, String> {
        let tmp = self.ingredients.clone();
        let idx = self.ingredients.iter().position(|ingredient: &Ingredient| {
            ingredient
                .id
                // if no id is present, unwrap and set it to -1 (invalid, will return "no idx found")
                .eq(&input_id)
        });
        match idx {
            Some(idx) => {
                return Ok(tmp[idx].clone());
            }
            None => return Err(String::from("No index found")),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(AsChangeset, Queryable, Selectable, Insertable)
)]
#[cfg_attr(not(target_arch="wasm32"), 
    diesel(table_name = recipe_users)
)]
pub struct NewUser {
    pub user_name: String,
    pub user_role: UserRole,
    pub user_pwd: String,
}
impl Default for NewUser {
    fn default() -> Self {
        Self {
            user_name: String::new(),
            user_role: UserRole::User,
            user_pwd: String::new(),
        }
    }
}
impl NewUser {
    /// Password validation function
    ///
    /// # TODO
    /// create Regex pwd validation    
    pub fn validate(&self, pwd: &str) -> Result<String, IOError> {
        return Ok(pwd.into());
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(AsChangeset, Queryable, Selectable, Identifiable,)
)]
#[cfg_attr(not(target_arch="wasm32"), 
    diesel(table_name = recipe_users)
)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub user_role: UserRole,
    pub user_pwd: String,
}
impl Default for User {
    fn default() -> Self {
        User {
            id: -1,
            user_name: String::new(),
            user_role: UserRole::User,
            user_pwd: String::new(),
        }
    }
}
impl User {
    /// modify Steps inside FullRecipe
    pub fn set_id(&mut self, user_id: i32) {
        self.id = user_id
    }
}

#[derive(PartialEq, Clone, Deserialize)]
pub struct UpdateUser {
    pub id: i32,
    pub user_name: Option<String>,
    pub user_role: Option<UserRole>,
    pub user_pwd: Option<String>,
}

impl Default for UpdateUser {
    fn default() -> Self {
        Self {
            id: -1,
            user_name: None,
            user_role: None,
            user_pwd: None,
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
impl Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => {
                write!(f, "admin")
            }
            UserRole::User => {
                write!(f, "user")
            }
        }
    }
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

#[derive(Deserialize, Serialize, Debug, PartialEq)]
/// Types of query that can be received from user as param
///
/// ex: https://.../?name=<RecipeName>
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// This struct represents possible Ok Values the API can generate
pub struct ApiOkResponse {
    pub msg: String,
}
