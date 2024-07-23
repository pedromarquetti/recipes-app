use std::{convert::Infallible, error::Error as StdError, num::ParseIntError, str::Utf8Error};

use bcrypt::BcryptError;
use diesel::result::DatabaseErrorKind;
use log::error;
use serde_json::json;

use warp::{
    body::BodyDeserializeError,
    hyper::StatusCode,
    reject::{self, InvalidQuery, MethodNotAllowed, Reject},
    reply::{self, WithStatus},
    Rejection, Reply,
};

use db::db_pool::{DieselError, R2D2Err};

/// convert any errors to my custom Error
pub fn convert_to_rejection<E>(error: E) -> Rejection
where
    E: Into<Error>,
{
    reject::custom(error.into())
}

pub async fn handle_rejection(err: Rejection) -> Result<WithStatus<Box<dyn Reply>>, Infallible> {
    if let Some(err) = err.find::<Error>() {
        error!("{:?}", err);
        Ok(err.convert_to_json())
    } else if let Some(err) = err.find::<InvalidQuery>() {
        error!("{:?}", err);

        Ok(Error::payload_error(format!(
            "Query Error: {:?} - {:?}",
            err.to_string(),
            err.source()
        ))
        .convert_to_json())
    } else if let Some(err) = err.find::<std::io::Error>() {
        error!("{:?}", err);

        Ok(Error::payload_error(format!(
            "Payload Error: 
                {:?} - msg {:?} ",
            err.to_string(),
            err.source()
        ))
        .convert_to_json())
    } else if let Some(err) = err.find::<BodyDeserializeError>() {
        // received invalid json body
        error!("{}", err);

        Ok(Error::payload_error(format!(
            "Payload Error: 
                {:?} - msg {:?} ",
            err.to_string(),
            err.source()
        ))
        .convert_to_json())
    } else if let Some(err) = err.find::<MethodNotAllowed>() {
        // Reject invalid HTTP req for specified path.
        error!("{}", err);

        Ok(Error::payload_error(format!(
            "Payload Error: 
                {:?} - msg {:?} ",
            err.to_string(),
            err.source()
        ))
        .convert_to_json())
    } else if err.is_not_found() {
        //received invalid json body
        error!("{:?}", err);

        Ok(Error::not_found(format!("Not Found!")).convert_to_json())
    } else {
        Ok(Error::internal_error(
            format!("Internal server error: {:?}", err),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .convert_to_json())
    }
}

#[derive(Debug)]
/// types of errors
enum ErrorKind {
    NotFound,
    UniqueViolation,
    DatabaseError,
    PayloadError,
    InternalServerError,
    UserAuthError,
}
#[derive(Debug)]
/// Custom error types
pub struct Error {
    kind: ErrorKind,
    status_code: StatusCode,
    msg: String,
}

impl Error {
    // handling errors
    pub fn not_found<S: Into<String>>(msg: S) -> Self {
        Self {
            kind: ErrorKind::NotFound,
            status_code: StatusCode::NOT_FOUND,
            msg: msg.into(),
        }
    }
    pub fn user_error<S: Into<String>>(msg: S, status_code: StatusCode) -> Self {
        Self {
            kind: ErrorKind::UserAuthError,
            status_code: status_code,
            msg: msg.into(),
        }
    }
    pub fn db_error<S: Into<String>>(msg: S) -> Self {
        Self {
            kind: ErrorKind::DatabaseError,
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            msg: msg.into(),
        }
    }
    pub fn unique_violation<S: Into<String>>(msg: S) -> Self {
        Self {
            kind: ErrorKind::UniqueViolation,
            status_code: StatusCode::CONFLICT,
            msg: msg.into(),
        }
    }
    /// Unknown error occured!
    pub fn internal_error<S: Into<String>>(msg: S, status_code: StatusCode) -> Self {
        Self {
            kind: ErrorKind::InternalServerError,
            status_code,
            msg: msg.into(),
        }
    }

    pub fn payload_error<S: Into<String>>(msg: S) -> Self {
        Self {
            kind: ErrorKind::PayloadError,
            status_code: StatusCode::BAD_REQUEST,
            msg: msg.into(),
        }
    }

    /// Convert Error to a valid json reply
    fn convert_to_json(&self) -> WithStatus<Box<dyn Reply>> {
        let msg = &self.msg;
        let body = match &self.kind {
            ErrorKind::NotFound => Box::new(reply::json(&json!({ "error": msg }))),
            ErrorKind::UniqueViolation => Box::new(reply::json(&json!({ "error": msg }))),
            ErrorKind::DatabaseError => Box::new(reply::json(&json!({ "error": msg }))),
            ErrorKind::PayloadError => Box::new(reply::json(&json!({ "error": msg }))),
            ErrorKind::InternalServerError => Box::new(reply::json(&json!({ "error": msg }))),
            ErrorKind::UserAuthError => Box::new(reply::json(&json!({"error":msg}))),
        };
        reply::with_status(body, self.status_code)
    }
}
impl Reject for Error {}

impl From<R2D2Err> for Error {
    fn from(value: R2D2Err) -> Self {
        Error::db_error(format!("DB Error: {}", value.to_string()))
    }
}
impl From<DieselError> for Error {
    fn from(value: DieselError) -> Self {
        match value {
            DieselError::DatabaseError(kind, msg) => match kind {
                DatabaseErrorKind::UniqueViolation => {
                    Error::unique_violation("Inserted value must be unique")
                }
                // other errors
                DatabaseErrorKind::ForeignKeyViolation => {
                    Error::db_error(format!("Foreign Key violation! {}", msg.message(),))
                }
                _ => Error::db_error(format!("Unexpected error!: {}", msg.message())),
            },
            err => Error::db_error(format!("Unexpected error! {}", err.to_string())),
        }
    }
}

impl From<BcryptError> for Error {
    fn from(value: BcryptError) -> Self {
        Error::internal_error(value.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Error::user_error(value.to_string(), StatusCode::BAD_REQUEST)
    }
}

impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        Error::payload_error(value.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::payload_error(value.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::payload_error(value.to_string())
    }
}
