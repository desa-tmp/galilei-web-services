use super::error::Result;
use sqlx::{postgres::PgPoolOptions, Postgres};

pub type Pool = sqlx::Pool<Postgres>;

pub async fn create_pool(database_url: &str, max_connections: u32) -> Result<Pool> {
  let pool = PgPoolOptions::new()
    .max_connections(max_connections)
    .connect(database_url)
    .await?;

  sqlx::migrate!("./migrations")
    .run(&pool)
    .await
    .map_err(|err| sqlx::Error::from(err))?;

  Ok(pool)
}
