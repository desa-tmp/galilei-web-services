use actix_web::{
  http::{header, StatusCode},
  HttpResponse, ResponseError,
};
use derive_more::{Display, Error, From};
use serde::Serialize;
use utoipa::{ToResponse, ToSchema};
use validator::ValidationErrors;

pub type Result<T, E = ApiError> = std::result::Result<T, E>;

#[derive(Debug, Display, Error, From)]
pub enum ApiError {
  #[display(fmt = "Requested resources not found")]
  NotFound,
  #[display(fmt = "The resource already exists")]
  AlreadyExists,
  #[display(fmt = "Validation error on fields: {_0}")]
  Validation(ValidationErrors),
  #[display(fmt = "An internal error occurred")]
  InternalError,
}

impl From<sqlx::Error> for ApiError {
  fn from(value: sqlx::Error) -> Self {
    match value {
      sqlx::Error::RowNotFound => Self::NotFound,
      sqlx::Error::Database(err) => match err.kind() {
        sqlx::error::ErrorKind::UniqueViolation => Self::AlreadyExists,
        sqlx::error::ErrorKind::ForeignKeyViolation => Self::NotFound,
        _ => Self::InternalError,
      },
      _ => Self::InternalError,
    }
  }
}

impl ResponseError for ApiError {
  fn error_response(&self) -> HttpResponse {
    let status_code = self.status_code();

    HttpResponse::build(status_code)
      .append_header(header::ContentType::json())
      .json(ErrorResponse {
        status_code: status_code.as_u16(),
        message: self.to_string(),
      })
  }

  fn status_code(&self) -> actix_web::http::StatusCode {
    match self {
      ApiError::NotFound => StatusCode::NOT_FOUND,
      ApiError::AlreadyExists => StatusCode::CONFLICT,
      ApiError::Validation(_) => StatusCode::BAD_REQUEST,
      ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
  status_code: u16,
  message: String,
}

#[derive(ToResponse)]
#[response(
  description = "Requested resources not found",
  content_type = "application/json"
)]
pub struct NotFoundResponse(#[to_schema] ErrorResponse);

#[derive(ToResponse)]
#[response(
  description = "The resource already exists",
  content_type = "application/json"
)]
pub struct AlreadyExistsResponse(#[to_schema] ErrorResponse);

#[derive(ToResponse)]
#[response(
  description = "The body of the request contains incorrect data",
  content_type = "application/json"
)]
pub struct ValidationResponse(#[to_schema] ErrorResponse);

#[derive(ToResponse)]
#[response(
  description = "An internal error occurred",
  content_type = "application/json"
)]
pub struct InternalErrorResponse(#[to_schema] ErrorResponse);
