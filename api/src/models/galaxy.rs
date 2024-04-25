use async_trait::async_trait;
use derive_more::From;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

use crate::database::{Connection, DbResult};
use crate::gen_update_data;

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

#[derive(Debug, From, Deserialize, Clone)]
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

  async fn all(conn: &mut Connection, ident: Self::OwnerIdent) -> DbResult<Vec<Self>> {
    let UserId(user_id) = ident;

    let galaxies = sqlx::query_as!(Galaxy, "SELECT * FROM galaxies WHERE user_id = $1", user_id)
      .fetch_all(conn)
      .await?;

    Ok(galaxies)
  }

  async fn create(
    conn: &mut Connection,
    ident: Self::OwnerIdent,
    data: Self::CreateData,
  ) -> DbResult<Self> {
    let UserId(user_id) = ident;
    let CreateGalaxyData { name } = data;

    let new_user = sqlx::query_as!(
      Galaxy,
      "INSERT INTO galaxies(name, user_id) VALUES ($1, $2) RETURNING *",
      name,
      user_id
    )
    .fetch_one(conn)
    .await?;

    Ok(new_user)
  }

  async fn update(
    conn: &mut Connection,
    ident: Self::ResourceIdent,
    data: Self::UpdateData,
  ) -> DbResult<Self> {
    let GalaxyPath(galaxy_id) = ident;
    let UpdateGalaxyData { name } = data;

    let updated_user = sqlx::query_as!(
      Galaxy,
      "UPDATE galaxies SET name = COALESCE($1, name) WHERE id = $2 RETURNING *",
      name,
      galaxy_id
    )
    .fetch_one(conn)
    .await?;

    Ok(updated_user)
  }

  async fn delete(conn: &mut Connection, ident: Self::ResourceIdent) -> DbResult<Self> {
    let GalaxyPath(galaxy_id) = ident;

    let deleted_user = sqlx::query_as!(
      Galaxy,
      "DELETE FROM galaxies WHERE id = $1 RETURNING *",
      galaxy_id
    )
    .fetch_one(conn)
    .await?;

    Ok(deleted_user)
  }
}
