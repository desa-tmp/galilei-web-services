use crate::{gen_update_data, prelude::*};
use async_trait::async_trait;
use derive_more::From;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

pub use super::{galaxy::GalaxyPath, CrudOperations};

#[derive(Debug, Serialize, ToSchema)]
pub struct Star {
  pub id: Uuid,
  #[schema(min_length = 1)]
  pub name: String,
  #[schema(format = Uri)]
  pub nebula: String,
  pub galaxy_id: Uuid,
}

gen_update_data! {
  UpdateStarData,
  #[derive(Debug, Deserialize, Validate, ToSchema)]
  pub struct CreateStarData {
    #[schema(min_length = 1)]
    #[validate(length(min = 1, message = "cannot be empty"))]
    name: String,
    #[schema(format = Uri)]
    #[validate(url(message = "must be a valid url"))]
    nebula: String,
  }
}

#[derive(Debug, From, Deserialize, IntoParams)]
#[into_params(names("galaxy_id", "star_id"), parameter_in = Path)]
pub struct StarPath(Uuid, Uuid);

#[async_trait]
impl CrudOperations for Star {
  type OwnerIdent = GalaxyPath;
  type ResourceIdent = StarPath;
  type CreateData = CreateStarData;
  type UpdateData = UpdateStarData;

  async fn all(pool: &Pool, ident: Self::OwnerIdent) -> sqlx::Result<Vec<Self>> {
    let GalaxyPath(galaxy_id) = ident;

    sqlx::query_as!(Star, "SELECT * FROM stars WHERE galaxy_id = $1", galaxy_id)
      .fetch_all(pool)
      .await
  }
  async fn create(
    pool: &Pool,
    ident: Self::OwnerIdent,
    data: Self::CreateData,
  ) -> sqlx::Result<Self> {
    let GalaxyPath(galaxy_id) = ident;
    let CreateStarData { name, nebula } = data;

    sqlx::query_as!(
      Star,
      "INSERT INTO stars(name, nebula, galaxy_id) VALUES ($1, $2, $3) RETURNING *",
      name,
      nebula,
      galaxy_id
    )
    .fetch_one(pool)
    .await
  }
  async fn update(
    pool: &Pool,
    ident: Self::ResourceIdent,
    data: Self::UpdateData,
  ) -> sqlx::Result<Self> {
    let StarPath(galaxy_id, star_id) = ident;
    let UpdateStarData { name, nebula } = data;

    sqlx::query_as!(
      Star,
      r#"
      UPDATE stars
      SET name = COALESCE($1, name),
        nebula = COALESCE($2, nebula)
      WHERE galaxy_id = $3 AND id = $4
      RETURNING *
    "#,
      name,
      nebula,
      galaxy_id,
      star_id
    )
    .fetch_one(pool)
    .await
  }
  async fn delete(pool: &Pool, ident: Self::ResourceIdent) -> sqlx::Result<Self> {
    let StarPath(galaxy_id, star_id) = ident;

    sqlx::query_as!(
      Star,
      "DELETE FROM stars WHERE galaxy_id = $1 AND id = $2 RETURNING *",
      galaxy_id,
      star_id
    )
    .fetch_one(pool)
    .await
  }
}
