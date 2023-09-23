use std::{collections::HashMap, convert::Infallible};

use db::{
    functions::recipe,
    structs::{FullRecipe, Recipe, Step},
};
use gloo_net::{http::Request, Error as GlooError};
use log::{debug, error};
use serde::{Deserialize, Deserializer};
use serde_json::{json, Value};

#[derive(Debug)]
pub enum ApiResponse {
    OkRecipe(FullRecipe),
    ErrorMessage(String),
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
        let recipe: Value = serde_json::from_str(&res).map_err(|e| {
            error!("an error occurred:{:?}", e.to_string());
            GlooError::SerdeError(e)
        })?;
        debug!(" json data {:?}", recipe);

        Ok(ApiResponse::OkRecipe(FullRecipe {
            recipe: Recipe {
                id: None,
                user_id: None,
                recipe_name: "()".into(),
                recipe_ingredients: vec![],
                recipe_observations: None,
            },
            steps: vec![],
        }))
    }
}
