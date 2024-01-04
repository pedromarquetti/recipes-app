use db::structs::{User, UserRole};
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
    pub user_id: i32,
    pub role: UserRole,
    pub exp: usize,
}

pub fn generate_token(user: User) -> Result<String, JWTError> {
    let claims = UserClaims {
        user_id: user.id.expect("Expected USER ID"),
        role: user.user_role,
        exp: 10000000000,
    };
    return encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret().as_ref()),
    );
}

pub fn validate_token<T: DeserializeOwned>(token: String) -> JWTResult<TokenData<T>> {
    return decode(
        &token,
        &DecodingKey::from_secret(get_secret().as_ref()),
        &Validation::new(Algorithm::HS256),
    );
}
