use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

use crate::auth::{AuthSecurity, Token};
use crate::database::{DbError, DbResult, Pool};

use super::galaxy::UserId;

pub struct Session {
  pub id: Uuid,
  pub token: String,
  pub expires: Option<NaiveDateTime>,
  pub user_id: Uuid,
}

impl Session {
  pub async fn verify_token(pool: &Pool, token: Token) -> DbResult<UserId> {
    let token_hash = token.hash()?;

    let row = sqlx::query!(
      "SELECT expires, user_id FROM sessions WHERE token = $1",
      token_hash
    )
    .fetch_one(pool)
    .await?;

    if let Some(expires) = row.expires {
      if expires.and_utc() > Utc::now() {
        return Err(DbError::NotFound);
      }
    }

    Ok(UserId::from(row.user_id))
  }

  pub async fn create(
    pool: &Pool,
    token: &Token,
    expires: Option<NaiveDateTime>,
    user_id: &Uuid,
  ) -> DbResult<Session> {
    let token_hash = token.hash()?;

    let session = sqlx::query_as!(
      Session,
      "INSERT INTO sessions(token, expires, user_id) VALUES ($1, $2, $3) RETURNING *",
      token_hash,
      expires,
      user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(session)
  }
}
