use actix_web::{
  http::{header::ContentType, StatusCode},
  HttpResponse, ResponseError,
};
use derive_more::{Display, Error, From};

#[derive(Debug, Display, From, Error)]
pub enum Error {
  #[display(fmt = "transaction slot not found; ensure to use the middleware")]
  MissingMiddleware,
  #[display(fmt = "multiple transaction extractors found in the handler/middleware")]
  MultipleExtractors,
  #[display(fmt = "{}", .0)]
  Database(sqlx::Error),
}

impl ResponseError for Error {
  fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
    HttpResponse::build(self.status_code())
      .content_type(ContentType::plaintext())
      .body("Internal Server Error")
  }

  fn status_code(&self) -> actix_web::http::StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
  }
}
