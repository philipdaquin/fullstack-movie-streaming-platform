use actix_web::{error::ResponseError, HttpResponse};
use async_graphql::ErrorExtensions;
use serde::Serialize;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum ServiceError {
    #[error("Could not find resource")]
    NotFound,

    #[error("User is not authorized")]
    Unauthorized,

    #[error("Not authorized to request the specified resource")]
    Forbidden,

    #[error("Incorrect credentials provided")]
    IncorrectCredentials,

    #[error("Anonymous users do not have access to this resource")]
    AnonymousError,

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Invalid token provided")]
    InvalidToken(String),

    #[error("A server error occurred")]
    DatabaseError,

    #[error("Internal Server Error")]
    ServerError(String),

    #[error("Internal Server Error")]
    PoisonConcurrencyError(String),

    #[error("Unexpected error occurred")]
    UnexpectedError,

    #[error("Provided data was malformed")]
    MalformedData,

    #[error(transparent)]
    CryptoError(#[from] argon2::Error),

    #[error("Rand Error: {0}")]
    RandError(String),

    #[error(transparent)]
    StrConversion(#[from] std::str::Utf8Error),
}

impl ErrorExtensions for ServiceError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{}", self)).extend_with(|_, e| match self {
            Self::BadRequest(error) => {
                e.set("status", 400);
                e.set("statusText", "BAD_REQUEST");
                e.set("details", error.to_string());
            }
            Self::Unauthorized | Self::IncorrectCredentials | Self::AnonymousError => {
                e.set("status", 401);
                e.set("statusText", "UNAUTHORIZED");
            }
            Self::InvalidToken(error) => {
                e.set("status", 401);
                e.set("statusText", "INVALID_TOKEN");
                e.set("details", error.to_string());
            }
            Self::Forbidden => {
                e.set("status", 403);
                e.set("statusText", "FORBIDDEN");
            }
            Self::NotFound => {
                e.set("status", 404);
                e.set("statusText", "NOT_FOUND");
            }
            Self::ServerError(error) => {
                e.set("status", 500);
                e.set("statusText", "SERVER_ERROR");
                e.set("context", error.to_string());
            }
            Self::UnexpectedError | Self::PoisonConcurrencyError(_) => {
                e.set("status", 500);
                e.set("statusText", "SERVER_ERROR");
            }
            _ => {}
        })
    }
}

#[derive(Debug, Serialize)]
struct Messages(Vec<String>);

impl From<Vec<&String>> for Messages {
    fn from(s: Vec<&String>) -> Self {
        Self(s.iter().map(|s| s.to_string()).collect::<Vec<String>>())
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::NotFound => HttpResponse::NotFound().finish(),
            Self::Unauthorized | Self::IncorrectCredentials => {
                HttpResponse::Unauthorized().finish()
            }
            Self::Forbidden => HttpResponse::Forbidden().finish(),
            // Self::InvalidToken(error) => {
            //     HttpResponse::Unauthorized().json::<Messages>(vec![error].into())
            // }
            // Self::ServerError(error) => {
            //     HttpResponse::InternalServerError().json::<Messages>(vec![error].into())
            // }
            Self::UnexpectedError => HttpResponse::InternalServerError().finish(),
            // Catch all, as most of the time we should be using GraphQL errors
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}

// @TODO add more precise match for `Unique constraint violated` error (ie. customer already exists)
impl From<sqlx::Error> for ServiceError {
    fn from(e: sqlx::Error) -> ServiceError {
        use sqlx::Error::*;

        match e {
            RowNotFound => ServiceError::NotFound,
            _ => {
                error!(err = ?e, "SQLx error occurred");
                ServiceError::DatabaseError
            }
        }
    }
}

impl From<serde_json::Error> for ServiceError {
    fn from(e: serde_json::Error) -> ServiceError {
        use serde_json::error::Category::*;
        error!(err = ?e, "JSON Serde error occurred");

        match e.classify() {
            Syntax | Data => ServiceError::MalformedData,
            _ => ServiceError::UnexpectedError,
        }
    }
}

impl From<tokio::task::JoinError> for ServiceError {
    fn from(e: tokio::task::JoinError) -> ServiceError {
        error!(
            err = ?e,
            was_cancelled = e.is_cancelled(),
            did_panic = e.is_panic(),
            "Tokio task join error occurred"
        );
        ServiceError::UnexpectedError
    }
}

impl From<rand::Error> for ServiceError {
    fn from(e: rand::Error) -> ServiceError {
        error!(
            err = ?e,
            error_code = ?e.code(),
            "Received Rand error"
        );
        ServiceError::RandError(e.to_string())
    }
}

