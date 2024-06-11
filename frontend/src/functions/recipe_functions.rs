use db::structs::{FullRecipe, Ingredient, Recipe, Step};
use gloo_net::{http::Request, Error as GlooError};
use serde_json::Value;

use super::{parse_api_response, ApiResponse};

/// View details about recipe
///
/// # Returns
///
/// 1. ok FullRecipe
/// 1. error message from backend
// pub async fn fetch_recipe(recipe_id: i32) -> Result<ApiResponse<FullRecipe, String>, GlooError> {
pub async fn fetch_recipe(recipe_id: i32) -> Result<ApiResponse<FullRecipe, String>, GlooError> {
    let req = Request::get(&format!("/api/get/recipe/?id={recipe_id}"))
        .send()
        .await?;

    let res: Value = req.json().await?;
    parse_api_response(res).await
}

pub async fn fuzzy_list_recipe(name: String) -> Result<Vec<Recipe>, GlooError> {
    // updating endpoint addresses
    let req = Request::get(&format!("/api/get/recipes/?name={}", name))
        .send()
        .await?;
    req.json::<Vec<Recipe>>().await
}

pub async fn create_recipe(recipe: Recipe) -> Result<ApiResponse<Recipe, String>, GlooError> {
    let req = Request::post("/api/create/recipe")
        .json(&recipe)?
        .send()
        .await?;
    let res: Value = req.json().await?;
    parse_api_response(res).await
}

pub async fn delete_recipe(recipe: Recipe) -> Result<ApiResponse<FullRecipe, String>, GlooError> {
    let req = Request::post("/api/delete/recipe")
        .json(&recipe)?
        .send()
        .await?;
    let res: Value = req.json().await?;
    parse_api_response(res).await
}

pub async fn create_ingredient(
    ingredient: Vec<Ingredient>,
) -> Result<ApiResponse<Ingredient, String>, GlooError> {
    let req = Request::post("/api/create/ingredient")
        .json(&ingredient)?
        .send()
        .await?;
    let res: Value = req.json().await?;
    parse_api_response(res).await
}

pub async fn create_step(step: Vec<Step>) -> Result<ApiResponse<Step, String>, GlooError> {
    let req = Request::post("/api/create/step")
        .json(&step)?
        .send()
        .await?;
    let res: Value = req.json().await?;
    parse_api_response(res).await
}
