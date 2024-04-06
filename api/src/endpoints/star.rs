use actix_web::{
  delete, get, post, put,
  web::{Json, Path},
  HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct Star {
  id: Uuid,
  name: String,
  nebula: String,
  galaxy_id: Uuid,
}

#[utoipa::path(
  params(
    ("galaxy_id" = Uuid, Path, description = "id of the galaxy where stars are contained")
  ),
  responses(
    (
      status = 200,
      description = "all stars in the galaxy",
      content_type = "application/json",
      body = Vec<Star>
    )
  )
)]
#[get("/galaxies/{galaxy_id}/stars")]
pub async fn list_all_stars(path_params: Path<Uuid>) -> impl Responder {
  let galaxy_id = path_params.into_inner();
  HttpResponse::Ok().body(format!("All stars in galaxy {galaxy_id}"))
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct CreateStarData {
  name: String,
  nebula: String,
}

#[utoipa::path(
  params(
    ("galaxy_id" = Uuid, Path, description = "id of the galaxy where the star will created")
  ),
  request_body(
    content = CreateStarData,
    description = "data for creating the star",
    content_type = "application/json"
  ),
  responses(
    (
      status = 200,
      description = "star successfully created",
      content_type = "application/json",
      body = Star
    )
  )
)]
#[post("/galaxies/{galaxy_id}/stars")]
pub async fn create_star(
  path_params: Path<Uuid>,
  Json(data): Json<CreateStarData>,
) -> impl Responder {
  let galaxy_id = path_params.into_inner();
  HttpResponse::Created().body(format!(
    "New star created in galaxy {galaxy_id} with data {:?}",
    data
  ))
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct UpdateStarData {
  name: Option<String>,
  nebula: Option<String>,
}

#[utoipa::path(
  params(
    ("galaxy_id" = Uuid, Path, description = "id of the galaxy where the star is contained"),
    ("star_id" = Uuid, Path, description = "id of the star to update")
  ),
  request_body(
    content = UpdateStarData,
    description = "data for updating the star",
    content_type = "application/json"
  ),
  responses(
    (
      status = 200,
      description = "star successfully updated",
      content_type = "application/json",
      body = Star
    )
  )
)]
#[put("/galaxies/{galaxy_id}/stars/{star_id}")]
pub async fn update_star(
  path_param: Path<(Uuid, Uuid)>,
  Json(data): Json<UpdateStarData>,
) -> impl Responder {
  let (galaxy_id, star_id) = path_param.into_inner();
  HttpResponse::Ok().body(format!(
    "Star {star_id} updated in galaxy {galaxy_id} with {:?}",
    data
  ))
}

#[utoipa::path(
  params(
    ("galaxy_id" = Uuid, Path, description = "id of the galaxy where the star is contained"),
    ("star_id" = Uuid, Path, description = "id of the star to delete")
  ),
  responses(
    (
      status = 200,
      description = "star successfully created",
      content_type = "application/json",
      body = Star
    )
  )
)]
#[delete("/galaxies/{galaxy_id}/stars/{star_id}")]
pub async fn delete_star(path_param: Path<(Uuid, Uuid)>) -> impl Responder {
  let (galaxy_id, star_id) = path_param.into_inner();
  HttpResponse::Ok().body(format!("Star {star_id} deleted in galaxy {galaxy_id}"))
}
