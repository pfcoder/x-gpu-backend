use crate::response::{ApiFailure, Failure};
use poem::{http::StatusCode, web::Json};
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    TokioRecvError(#[from] tokio::sync::oneshot::error::RecvError),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    //#[error(transparent)]
    //PoemError(#[from] poem::Error),
    #[error("wrong credentials")]
    WrongCredentials,
    #[error("password doesn't match")]
    WrongPassword,
    #[error("email is already taken")]
    DuplicateUserEmail,
    #[error("name is already taken")]
    DuplicateUserName,
    #[error("name is already taken")]
    DuplicateVcTpltName,
    #[error("DID generate error")]
    DidGenerateError,
}
pub type Result<T> = std::result::Result<T, Error>;

pub type ApiError = (StatusCode, Json<Value>);
pub type ApiResult<T> = std::result::Result<T, ApiError>;

impl From<Error> for ApiError {
    fn from(err: Error) -> Self {
        let status = match err {
            Error::WrongCredentials => StatusCode::UNAUTHORIZED,
            Error::ValidationError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        //let payload = json!({"message": err.to_string()});
        let payload = json!(ApiFailure {
            api_version: Default::default(),
            body: Failure {
                code: status.as_u16(),
                message: err.to_string()
            }
        });
        (status, Json(payload))
    }
}
