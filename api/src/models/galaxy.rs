use crate::{gen_update_data, prelude::*};
use async_trait::async_trait;
use derive_more::{From, FromStr};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

pub use super::CrudOperations;

#[derive(Debug, Serialize, ToSchema)]
pub struct Galaxy {
  pub id: Uuid,
  #[schema(min_length = 1)]
  pub name: String,
  pub user_id: Uuid,
}

gen_update_data! {
  UpdateGalaxyData,
  #[derive(Debug, Deserialize, Validate, ToSchema)]
  pub struct CreateGalaxyData {
    #[schema(min_length = 1)]
    #[validate(length(min = 1, message = "cannot be empty"))]
    name: String,
  }
}

#[derive(Debug, FromStr, Deserialize)]
pub struct UserId(Uuid);

#[derive(Debug, From, Deserialize, IntoParams)]
#[into_params(names("galaxy_id"), parameter_in = Path)]
pub struct GalaxyPath(pub Uuid);

#[async_trait]
impl CrudOperations for Galaxy {
  type OwnerIdent = UserId;
  type ResourceIdent = GalaxyPath;
  type CreateData = CreateGalaxyData;
  type UpdateData = UpdateGalaxyData;

  async fn all(pool: &Pool, ident: Self::OwnerIdent) -> sqlx::Result<Vec<Self>> {
    let UserId(user_id) = ident;

    sqlx::query_as!(Galaxy, "SELECT * FROM galaxies WHERE user_id = $1", user_id)
      .fetch_all(pool)
      .await
  }

  async fn create(
    pool: &Pool,
    ident: Self::OwnerIdent,
    data: Self::CreateData,
  ) -> sqlx::Result<Self> {
    let UserId(user_id) = ident;
    let CreateGalaxyData { name } = data;

    sqlx::query_as!(
      Galaxy,
      "INSERT INTO galaxies(name, user_id) VALUES ($1, $2) RETURNING *",
      name,
      user_id
    )
    .fetch_one(pool)
    .await
  }

  async fn update(
    pool: &Pool,
    ident: Self::ResourceIdent,
    data: Self::UpdateData,
  ) -> sqlx::Result<Self> {
    let GalaxyPath(galaxy_id) = ident;
    let UpdateGalaxyData { name } = data;

    sqlx::query_as!(
      Galaxy,
      "UPDATE galaxies SET name = COALESCE($1, name) WHERE id = $2 RETURNING *",
      name,
      galaxy_id
    )
    .fetch_one(pool)
    .await
  }

  async fn delete(pool: &Pool, ident: Self::ResourceIdent) -> sqlx::Result<Self> {
    let GalaxyPath(galaxy_id) = ident;

    sqlx::query_as!(
      Galaxy,
      "DELETE FROM galaxies WHERE id = $1 RETURNING *",
      galaxy_id
    )
    .fetch_one(pool)
    .await
  }
}
