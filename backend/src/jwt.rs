use db::structs::User;
use jsonwebtoken::{
    decode, encode,
    errors::{Error as JWTError, Result as JWTResult},
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::env;

fn get_secret() -> String {
    return env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY not found!");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub user_name: String,
    pub role: String,
    pub exp: usize,
}

pub fn generate_token(user: User) -> Result<String, JWTError> {
    let claims = UserClaims {
        user_name: user.user_name,
        role: user.user_role,
        exp: 10000000000,
    };
    return encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret()),
    );
}

pub fn validate_token(token: String) -> JWTResult<TokenData<T>> {
    return decode(
        &token,
        &EncodingKey::from_secret(get_secret()),
        &Validation::new(Algorithm::HS256),
    );
}
