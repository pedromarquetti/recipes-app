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
        .json(&json!({
            "id":recipe_id,
            "recipe_name": "",
            "recipe_ingredients": [""],
        }))?
        .send()
        .await?;

    let res: Value = req.json().await?;

    if let Some(err) = res.get("error") {
        // recipe not found
        error!("recipe not found!");
        Ok(ApiResponse::ErrorMessage(
            serde_json::from_value(err.clone()).map_err(|e| {
                error!("an error occurred:{:?}", e.to_string());
                GlooError::SerdeError(e)
            })?,
        ))
    } else {
        Ok(ApiResponse::OkRecipe(serde_json::from_value(res).map_err(
            |e| {
                error!("an error occurred:{:?}", e.to_string());
                GlooError::SerdeError(e)
            },
        )?))
    }
}
