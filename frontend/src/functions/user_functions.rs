use db::structs::{NewRecipe, NewUser, User};
use gloo_net::{http::Request, Error as GlooError};
use serde_json::Value;

use super::{parse_api_response, ApiResponse};

pub async fn login_user(user: User) -> Result<ApiResponse<NewRecipe, String>, GlooError> {
    let req = Request::post(&format!("/api/login/user"))
        .json(&user)?
        .send()
        .await?;

    let res: Value = req.json().await?;
    parse_api_response(res).await
}

pub async fn create_user(user: NewUser) -> Result<ApiResponse<NewRecipe, String>, GlooError> {
    let req = Request::post(&format!("/api/create/user"))
        .json(&user)?
        .send()
        .await?;

    let res: Value = req.json().await?;
    parse_api_response(res).await
}
