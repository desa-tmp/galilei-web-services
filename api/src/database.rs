use sqlx::{postgres::PgPoolOptions, Postgres};
use std::sync::Arc;

use crate::auth;

pub type Pool = sqlx::Pool<Postgres>;
pub type DbResult<T, E = DbError> = std::result::Result<T, E>;

pub async fn create_pool(database_url: &str, max_connections: u32) -> DbResult<Arc<Pool>> {
  let pool = PgPoolOptions::new()
    .max_connections(max_connections)
    .connect(database_url)
    .await?;

  sqlx::migrate!("./migrations")
    .run(&pool)
    .await
    .map_err(|err| sqlx::Error::from(err))?;

  Ok(Arc::new(pool))
}

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
