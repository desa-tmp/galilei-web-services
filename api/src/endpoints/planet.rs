use actix_web::{
  delete, get, post, put,
  web::{Json, Path},
  HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct Planet {
  id: Uuid,
  name: String,
  capacity: u64,
  star_id: Option<Uuid>,
  galaxy_id: Uuid,
}

#[utoipa::path(
  params(
    ("galaxy_id" = Uuid, Path, description = "id of the galaxy where planets are contained")
  ),
  responses(
    (
      status = 200,
      description = "all planets in the galaxy",
      content_type = "application/json",
      body = Vec<Planet>
    )
  )
)]
#[get("/galaxies/{galaxy_id}/planets")]
pub async fn list_all_planets(path_params: Path<Uuid>) -> impl Responder {
  let galaxy_id = path_params.into_inner();
  HttpResponse::Ok().body(format!("All planets in galaxy {galaxy_id}"))
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct CreatePlanetData {
  name: String,
  capacity: u64,
  star_id: Option<Uuid>,
}

#[utoipa::path(
  params(
    ("galaxy_id" = Uuid, Path, description = "id of the galaxy where the planet will created")
  ),
  request_body(
    content = CreatePlanetData,
    description = "data for creating the planet",
    content_type = "application/json"
  ),
  responses(
    (
      status = 200,
      description = "planet successfully created",
      content_type = "application/json",
      body = Planet
    )
  )
)]
#[post("/galaxies/{galaxy_id}/planets")]
pub async fn create_planet(
  path_params: Path<Uuid>,
  Json(data): Json<CreatePlanetData>,
) -> impl Responder {
  let galaxy_id = path_params.into_inner();
  HttpResponse::Created().body(format!(
    "New planet created in galaxy {galaxy_id} with data {:?}",
    data
  ))
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct ConnectPlanetToStar {
  id: Option<Uuid>,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct UpdatePlanetData {
  name: Option<String>,
  capacity: Option<u64>,
  star: Option<ConnectPlanetToStar>,
}

#[utoipa::path(
  params(
    ("galaxy_id" = Uuid, Path, description = "id of the galaxy where the planet is contained"),
    ("planet_id" = Uuid, Path, description = "id of the planet to update")
  ),
  request_body(
    content = UpdatePlanetData,
    description = "data for updating the planet",
    content_type = "application/json"
  ),
  responses(
    (
      status = 200,
      description = "planet successfully updated",
      content_type = "application/json",
      body = Planet
    )
  )
)]
#[put("/galaxies/{galaxy_id}/planets/{planet_id}")]
pub async fn update_planet(
  path_param: Path<(Uuid, Uuid)>,
  Json(data): Json<UpdatePlanetData>,
) -> impl Responder {
  let (galaxy_id, planet_id) = path_param.into_inner();
  HttpResponse::Ok().body(format!(
    "Planet {planet_id} updated in galaxy {galaxy_id} with {:?}",
    data
  ))
}

#[utoipa::path(
  params(
    ("galaxy_id" = Uuid, Path, description = "id of the galaxy where the planet is contained"),
    ("planet_id" = Uuid, Path, description = "id of the planet to delete")
  ),
  responses(
    (
      status = 200,
      description = "planet successfully created",
      content_type = "application/json",
      body = Planet
    )
  )
)]
#[delete("/galaxies/{galaxy_id}/planets/{planet_id}")]
pub async fn delete_planet(path_param: Path<(Uuid, Uuid)>) -> impl Responder {
  let (galaxy_id, planet_id) = path_param.into_inner();
  HttpResponse::Ok().body(format!("Planet {planet_id} deleted in galaxy {galaxy_id}"))
}
