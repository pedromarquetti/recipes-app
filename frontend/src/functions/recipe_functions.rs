use std::{collections::HashMap, convert::Infallible};

use db::structs::{FullRecipe, Recipe, Step};
use gloo_net::{http::Request, Error as GlooError};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug)]
pub enum ApiResponse {
    OkRecipe(FullRecipe),
    ErrorMessage(String),
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TestRecipe {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub recipe_name: String,
    pub recipe_ingredients: Vec<String>,
    pub recipe_observations: Option<Vec<String>>,
}
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct TestStep {
    pub id: Option<i32>,
    pub recipe_id: i32,
    pub step_name: String,
    pub step_instruction: String,
    pub step_duration_min: i32,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
/// temporary fix for a bug i was having where i couldn't use Recipe struct from /db/
pub struct TestFullRecipe {
    pub recipe: TestRecipe,
    pub steps: Vec<TestStep>,
}

/// View details about recipe
///
/// # Returns
///
/// 1. ok recipe
/// 1. error message from backend
// pub async fn fetch_recipe(recipe_id: i32) -> Result<ApiResponse, GlooError> {
pub async fn fetch_recipe(recipe_id: i32) -> Result<ApiResponse, GlooError> {
    let req = Request::post("http://localhost:3000/api/view/recipe/")
        // .json::<FullRecipe>(&FullRecipe {
        //     recipe: Recipe {
        //         id: Some(recipe_id),
        //         user_id: None,
        //         recipe_name: "".into(),
        //         recipe_ingredients: vec![],
        //         recipe_observations: None,
        //     },
        //     steps: vec![],
        // })?
        .json(&json!({
            "id":recipe_id,
            "recipe_name": "",
            "recipe_ingredients": [""],
        }))?
        .send()
        .await?;

    // let res = req.json::<Value>().await?;
    let res = req.text().await?;

    let json: Value = serde_json::from_str(&res).map_err(|e| {
        error!("Parsing response json error occurred: {:?}", e.to_string());
        GlooError::SerdeError(e)
    })?;

    if let Some(err) = json.get("error") {
        error!("recipe not found!");
        let error: String = serde_json::from_value(err.clone()).map_err(|e| {
            error!("an error occurred:{:?}", e.to_string());
            GlooError::SerdeError(e)
        })?;
        debug!("{:?}", error);
        // recipe not found
        Ok(ApiResponse::ErrorMessage(error))
    } else {
        let recipe: FullRecipe = serde_json::from_value(json).map_err(|e| {
            error!("an error occurred:{:?}", e.to_string());
            GlooError::SerdeError(e)
        })?;
        debug!(" json data {:?}", recipe);
        Ok(ApiResponse::OkRecipe(FullRecipe {
            recipe: Recipe {
                id: recipe.recipe.id,
                user_id: recipe.recipe.user_id,
                recipe_name: recipe.recipe.recipe_name,
                recipe_ingredients: recipe.recipe.recipe_ingredients,
                recipe_observations: recipe.recipe.recipe_observations,
            },
            steps: vec![],
        }))
    }
}
