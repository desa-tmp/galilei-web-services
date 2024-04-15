use crate::{gen_update_data, prelude::*};
use async_trait::async_trait;
use derive_more::From;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

pub use super::{galaxy::GalaxyPath, CrudOperations};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Planet {
  pub id: Uuid,
  #[schema(min_length = 1)]
  pub name: String,
  #[schema(minimum = 0, maximum = 2147483647)]
  pub capacity: i32,
  pub star_id: Option<Uuid>,
  pub galaxy_id: Uuid,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ConnectPlanetToStar {
  id: Option<Uuid>, // id of the star to connect
}

gen_update_data! {
  UpdatePlanetData,
  #[derive(Debug, Deserialize, Validate, ToSchema)]
  pub struct CreatePlanetData {
    #[schema(min_length = 1)]
    #[validate(length(min = 1, message = "cannot be empty"))]
    name: String,
    #[schema(minimum = 0, maximum = 2147483647)]
    #[validate(range(min = 0, max = 2147483647, message = "capacity must be between 0 and 2147483647"))]
    capacity: i32,
    star: ConnectPlanetToStar,
  }
}

#[derive(Debug, From, Deserialize, IntoParams)]
#[into_params(names("galaxy_id", "planet_id"), parameter_in = Path)]
pub struct PlanetPath(Uuid, Uuid);

#[async_trait]
impl CrudOperations for Planet {
  type OwnerIdent = GalaxyPath;
  type ResourceIdent = PlanetPath;
  type CreateData = CreatePlanetData;
  type UpdateData = UpdatePlanetData;

  async fn all(pool: &Pool, ident: Self::OwnerIdent) -> sqlx::Result<Vec<Self>> {
    let GalaxyPath(galaxy_id) = ident;

    sqlx::query_as!(
      Planet,
      "SELECT * FROM planets WHERE galaxy_id = $1",
      galaxy_id
    )
    .fetch_all(pool)
    .await
  }
  async fn create(
    pool: &Pool,
    ident: Self::OwnerIdent,
    data: Self::CreateData,
  ) -> sqlx::Result<Self> {
    let GalaxyPath(galaxy_id) = ident;
    let CreatePlanetData {
      name,
      capacity,
      star,
    } = data;

    sqlx::query_as!(
      Planet,
      "INSERT INTO planets(name, capacity, star_id, galaxy_id) VALUES ($1, $2, $3, $4) RETURNING *",
      name,
      capacity,
      star.id,
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
    let PlanetPath(galaxy_id, planet_id) = ident;
    let UpdatePlanetData {
      name,
      capacity,
      star,
    } = data;

    let update_star = star.is_some();
    let star_id = star.map(|con| con.id).unwrap_or(None);

    sqlx::query_as!(
      Planet,
      r#"
      UPDATE planets
      SET name = COALESCE($1, name),
        capacity = COALESCE($2, capacity),
        star_id = (CASE WHEN $3 = true THEN $4 ELSE star_id END)
      WHERE galaxy_id = $5 AND id = $6
      RETURNING *
    "#,
      name,
      capacity,
      update_star,
      star_id,
      galaxy_id,
      planet_id
    )
    .fetch_one(pool)
    .await
  }
  async fn delete(pool: &Pool, ident: Self::ResourceIdent) -> sqlx::Result<Self> {
    let PlanetPath(galaxy_id, planet_id) = ident;

    sqlx::query_as!(
      Planet,
      "DELETE FROM planets WHERE galaxy_id = $1 AND id = $2 RETURNING *",
      galaxy_id,
      planet_id
    )
    .fetch_one(pool)
    .await
  }
}
