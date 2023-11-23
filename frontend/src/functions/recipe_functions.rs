use std::fmt::Debug;

use db::structs::{FullRecipe, Recipe};
use gloo_net::{http::Request, Error as GlooError};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug)]
/// Possible Backend Responses
pub enum ApiResponse<R>
where
    R: for<'a> Deserialize<'a> + Serialize + PartialEq + Clone + Debug,
{
    OkRecipe(R),
    ErrorMessage(String),
}
/// View details about recipe
///
/// # Returns
///
/// 1. ok FullRecipe
/// 1. error message from backend
pub async fn fetch_recipe(recipe_id: i32) -> Result<ApiResponse<FullRecipe>, GlooError> {
    let req = Request::post("/api/get/recipes")
        .json(&json!({
            "id":recipe_id,
            "recipe_name": "",
            "recipe_ingredients": [""],
        }))?
        .send()
        .await?;

    let res: Value = req.json().await?;
    parse_api_response::<FullRecipe>(res).await
}

pub async fn fuzzy_list_recipe(name: String) -> Result<Vec<Recipe>, GlooError> {
    // updating endpoint addresses
    let req = Request::post("/api/get/recipes")
        .json::<Recipe>(&Recipe {
            id: None,
            recipe_name: name,
            user_id: None,
            recipe_observations: None,
        })?
        .send()
        .await?;
    req.json::<Vec<Recipe>>().await
}

pub async fn create_recipe(recipe: Recipe) -> Result<ApiResponse<Recipe>, GlooError> {
    let req = Request::post("/api/create/recipe")
        .json(&recipe)?
        .send()
        .await?;
    let res: Value = req.json().await?;
    parse_api_response::<Recipe>(res).await
}

async fn parse_api_response<R>(res: Value) -> Result<ApiResponse<R>, GlooError>
where
    R: for<'a> Deserialize<'a> + Serialize + PartialEq + Clone + Debug,
{
    debug!("{:#?}", res);
    if let Some(err) = res.get("error") {
        // recipe not found
        error!("server responded with error: {err}");
        Ok(ApiResponse::ErrorMessage(
            serde_json::from_value(err.clone()).map_err(|e| {
                error!("an error occurred:{:?}", e.to_string());
                GlooError::SerdeError(e)
            })?,
        ))
    } else {
        Ok(ApiResponse::OkRecipe(serde_json::from_value(res).map_err(
            |e| {
                error!(
                    "fetch ok, but an error occurred (probably trying to parse):{:?}",
                    e.to_string()
                );
                GlooError::SerdeError(e)
            },
        )?))
    }
}
