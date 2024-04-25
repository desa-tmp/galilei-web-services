use sqlx::{postgres::PgPoolOptions, Postgres};
use std::sync::Arc;

use crate::error::ApiError;

mod error;

pub use error::{DbError, Operation};

pub type Transaction = transaction::Transaction<Postgres, ApiError>;
pub type TransactionService = transaction::TransactionService<Postgres, ApiError>;

pub type Pool = sqlx::Pool<Postgres>;
pub type Connection = sqlx::PgConnection;
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
