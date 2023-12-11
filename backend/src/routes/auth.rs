use db::structs::UserClaims;
use jsonwebtoken::TokenData;
use warp::hyper::StatusCode;
use warp::{Filter, Rejection};

use crate::error::{convert_to_rejection, Error};

pub fn auth() -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::cookie::optional("jwt").and_then(check_header)
}
pub async fn check_header(cookie: Option<String>) -> Result<(), Rejection> {
    if let Some(cookie_val) = cookie {
        let token: TokenData<UserClaims> =
            validate_token(cookie_val).map_err(convert_to_rejection)?;
        if token.claims.role == "adm" {
            return Ok(());
        } else {
            return Err(Error::user_error(
                "User Can't view page",
                StatusCode::UNAUTHORIZED,
            ));
        }
    } else {
        return Err(Error::user_error(
            "Cookie not found!",
            StatusCode::UNAUTHORIZED,
        ));
    }
}
