use crate::auth;

#[derive(Debug)]
pub enum Operation {
  All,
  Get,
  Create,
  Update,
  Delete,
}

#[derive(Debug)]
pub enum DbError {
  NotFound,
  AlreadyExists,
  OperationNotImplemented(Operation),
  Internal(sqlx::Error),
  Auth(auth::AuthError),
}

impl From<sqlx::Error> for DbError {
  fn from(value: sqlx::Error) -> Self {
    match value {
      sqlx::Error::RowNotFound => DbError::NotFound,
      sqlx::Error::Database(err) => match err.kind() {
        sqlx::error::ErrorKind::UniqueViolation => DbError::AlreadyExists,
        sqlx::error::ErrorKind::ForeignKeyViolation => DbError::NotFound,
        _ => DbError::Internal(sqlx::Error::Database(err)),
      },
      err => DbError::Internal(err),
    }
  }
}

impl From<auth::AuthError> for DbError {
  fn from(value: auth::AuthError) -> Self {
    DbError::Auth(value)
  }
}
