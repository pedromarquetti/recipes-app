use crate::jwt::{validate_token, UserClaims};
use jsonwebtoken::TokenData;
use warp::hyper::StatusCode;
use warp::{Filter, Rejection};

use crate::error::{convert_to_rejection, Error};

pub fn auth() -> impl Filter<Extract = (Option<UserClaims>,), Error = Rejection> + Clone + Copy {
    warp::cookie::optional::<String>("jwt")
        .and_then(check_header)
        .untuple_one()
}

async fn check_header(cookie: Option<String>) -> Result<(Option<UserClaims>,), Rejection> {
    if let Some(cookie_val) = cookie {
        // jwt cookie found!
        let token: TokenData<UserClaims> =
            validate_token(cookie_val).map_err(convert_to_rejection)?;
        return Ok((Some(token.claims),));
    } else {
        return Ok((None,));
    }
}
