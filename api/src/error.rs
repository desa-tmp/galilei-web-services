use actix_web::{
  http::{header, StatusCode},
  HttpResponse, ResponseError,
};
use derive_more::{Display, Error, From};
use serde::Serialize;
use utoipa::{ToResponse, ToSchema};
use validator::ValidationErrors;

use crate::{auth, database};

pub type ApiResult<T, E = ApiError> = std::result::Result<T, E>;

#[derive(Debug, Display, Error, From)]
pub enum ApiError {
  #[display(fmt = "User not authorized")]
  Unauthorize,
  #[display(fmt = "Requested resources not found")]
  NotFound,
  #[display(fmt = "The resource already exists")]
  AlreadyExists,
  #[display(fmt = "Validation error on fields: {_0}")]
  Validation(ValidationErrors),
  #[display(fmt = "An internal error occurred")]
  InternalError,
}

impl From<database::DbError> for ApiError {
  fn from(value: database::DbError) -> Self {
    match value {
      database::DbError::NotFound => ApiError::NotFound,
      database::DbError::AlreadyExists => ApiError::AlreadyExists,
      database::DbError::Auth(err) => ApiError::from(err),
      _ => ApiError::InternalError,
    }
  }
}

impl From<transaction::Error> for ApiError {
  fn from(_value: transaction::Error) -> Self {
    ApiError::InternalError
  }
}

impl From<auth::AuthError> for ApiError {
  fn from(value: auth::AuthError) -> Self {
    match value {
      auth::AuthError::Invalid => ApiError::Unauthorize,
      auth::AuthError::Other(_) => ApiError::InternalError,
    }
  }
}

impl From<kube::Error> for ApiError {
  fn from(_value: kube::Error) -> Self {
    ApiError::InternalError
  }
}

impl ResponseError for ApiError {
  fn error_response(&self) -> HttpResponse {
    let status_code = self.status_code();

    HttpResponse::build(status_code)
      .append_header(header::ContentType::json())
      .json(ErrorMessage {
        status_code: status_code.as_u16(),
        message: self.to_string(),
      })
  }

  fn status_code(&self) -> actix_web::http::StatusCode {
    match self {
      ApiError::Unauthorize => StatusCode::UNAUTHORIZED,
      ApiError::NotFound => StatusCode::NOT_FOUND,
      ApiError::AlreadyExists => StatusCode::CONFLICT,
      ApiError::Validation(_) => StatusCode::BAD_REQUEST,
      ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

#[derive(Serialize, ToSchema)]
pub struct ErrorMessage {
  status_code: u16,
  message: String,
}

#[derive(ToResponse)]
#[response(description = "User not authorized", content_type = "application/json")]
pub struct UnauthorizeResponse(ErrorMessage);

#[derive(ToResponse)]
#[response(
  description = "Requested resources not found",
  content_type = "application/json"
)]
pub struct NotFoundResponse(ErrorMessage);

#[derive(ToResponse)]
#[response(
  description = "The resource already exists",
  content_type = "application/json"
)]
pub struct AlreadyExistsResponse(ErrorMessage);

#[derive(ToResponse)]
#[response(
  description = "The body of the request contains incorrect data",
  content_type = "application/json"
)]
pub struct ValidationResponse(ErrorMessage);

#[derive(ToResponse)]
#[response(
  description = "An internal error occurred",
  content_type = "application/json"
)]
pub struct InternalErrorResponse(ErrorMessage);
