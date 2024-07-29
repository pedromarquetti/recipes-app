pub mod recipe_functions;
pub mod user_functions;

use db::structs::RecipeTrait;
use gloo_net::Error as GlooError;
use serde::Deserialize;
use serde_json::Value;
use std::fmt::{Debug, Display};

#[derive(Debug)]
/// # Possible Backend Responses
///
/// OkRecipe: Recipe (or fullRecipe) was fetched successfully
///
/// ApiMessage: Generic message from the backend
///
/// ApiError: Error message from the backend
pub enum ApiResponse<R, M>
where
    R: for<'a> Deserialize<'a> + RecipeTrait,
    M: for<'a> Deserialize<'a> + Display + Debug + PartialEq + Clone,
{
    OkPart(R),
    ApiMessage(M),
    ApiError(M),
}
pub async fn parse_api_response<R, M>(res: Value) -> Result<ApiResponse<R, M>, GlooError>
where
    R: for<'a> Deserialize<'a> + RecipeTrait,
    M: for<'a> Deserialize<'a> + Display + Debug + PartialEq + Clone,
{
    if let Some(err) = res.get("error") {
        // err key found in response

        Ok(ApiResponse::ApiError(
            serde_json::from_value::<M>(err.clone()).map_err(|e| GlooError::SerdeError(e))?,
        ))
    } else if let Some(msg) = res.get("msg") {
        // handles generic messages from the backend
        Ok(ApiResponse::ApiMessage(
            serde_json::from_value::<M>(msg.clone()).map_err(|e| {
                // handling parse error

                GlooError::SerdeError(e)
            })?,
        ))
    } else {
        // if no key is found, the try parsing the response as a RecipeTrait
        Ok(ApiResponse::OkPart(
            serde_json::from_value::<R>(res).map_err(|e| GlooError::SerdeError(e))?,
        ))
    }
}
