pub mod recipe_functions;

use db::structs::RecipeTrait;
use gloo_net::Error as GlooError;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Debug, Display};

#[derive(Debug)]
/// Possible Backend Responses
/// OkRecipe: Recipe was fetched successfully
/// ApiMessage: Generic message from the backend
/// ApiError: Error message from the backend
pub enum ApiResponse<R, M>
where
    R: for<'a> Deserialize<'a> + Serialize + PartialEq + Clone + Debug,
    M: for<'a> Deserialize<'a> + Display + Debug + PartialEq + Clone,
{
    OkRecipe(R),
    ApiMessage(M),
    ApiError(M),
}
pub async fn parse_api_response<R, M>(res: Value) -> Result<ApiResponse<R, M>, GlooError>
where
    R: for<'a> Deserialize<'a> + Serialize + RecipeTrait + Clone + PartialEq + Debug,
    M: for<'a> Deserialize<'a> + Display + Debug + PartialEq + Clone,
{
    if let Some(err) = res.get("error") {
        // err key found in response
        error!("server responded with error: {err}");
        Ok(ApiResponse::ApiError(
            serde_json::from_value(err.clone()).map_err(|e| {
                error!("an error occurred:{:?}", e.to_string());
                GlooError::SerdeError(e)
            })?,
        ))
    } else if let Some(msg) = res.get("msg") {
        // parsing msg key
        debug!("server responded with message: {msg}");
        // handles generic messages from the backend
        Ok(ApiResponse::OkRecipe(
            serde_json::from_value::<R>(msg.clone()).map_err(|e| {
                // handling parse error
                error!("an error occurred: {:?}", e.to_string());
                GlooError::SerdeError(e)
            })?,
        ))
    } else {
        Ok(ApiResponse::ApiMessage(
            serde_json::from_value(res).map_err(|e| {
                error!(
                    "fetch ok, but an error occurred (probably trying to parse):{:?}",
                    e.to_string()
                );
                GlooError::SerdeError(e)
            })?,
        ))
    }
}
