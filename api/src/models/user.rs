use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::auth::{AuthSecurity, Password};
use crate::database::{Connection, DbResult};

#[derive(Debug, Serialize, ToSchema)]
pub struct User {
  pub id: Uuid,
  pub name: String,
  #[serde(skip_serializing)]
  pub password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct Credentials {
  #[validate(length(min = 1, message = "cannot be empty"))]
  username: String,
  #[serde(skip_serializing, flatten)]
  #[validate(nested)]
  password: Password,
}

impl Credentials {
  pub fn username_ref(&self) -> &str {
    &self.username
  }
}

impl User {
  pub async fn verify_credentials(
    conn: &mut Connection,
    credentials: Credentials,
  ) -> DbResult<User> {
    let Credentials { username, password } = credentials;

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE name = $1", &username)
      .fetch_one(conn)
      .await?;

    password.verify(&user.password)?;

    Ok(user)
  }

  pub async fn create(conn: &mut Connection, credentials: Credentials) -> DbResult<User> {
    let Credentials { username, password } = credentials;

    let password_hash = password.hash()?;

    let new_user = sqlx::query_as!(
      User,
      "INSERT INTO users(name, password) VALUES ($1, $2) RETURNING *",
      username,
      password_hash
    )
    .fetch_one(conn)
    .await?;

    Ok(new_user)
  }

  pub async fn get_by_id(conn: &mut Connection, user_id: &Uuid) -> DbResult<User> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
      .fetch_one(conn)
      .await?;

    Ok(user)
  }
}
