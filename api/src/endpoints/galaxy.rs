use actix_web::{
  delete, get, post, put,
  web::{Json, Path},
  HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct Galaxy {
  id: Uuid,
  name: String,
  description: String,
  user_id: Uuid,
}

#[utoipa::path(
  responses(
    (
      status = 200,
      description = "all user galaxies",
      content_type = "application/json",
      body = Vec<Galaxy>
    )
  )
)]
#[get("/galaxies")]
pub async fn list_all_galaxies() -> impl Responder {
  HttpResponse::Ok().body("All galaxies")
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct CreateGalaxyData {
  name: String,
  description: String,
}

#[utoipa::path(
  request_body(
    content = CreateGalaxyData,
    description = "data for creating the galaxy",
    content_type = "application/json"
  ),
  responses(
    (
      status = 200,
      description = "galaxy successfully created",
      content_type = "application/json",
      body = Galaxy
    )
  )
)]
#[post("/galaxies")]
pub async fn create_galaxy(Json(data): Json<CreateGalaxyData>) -> impl Responder {
  HttpResponse::Created().body(format!("New galaxy created with data {:?}", data))
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct UpdateGalaxyData {
  name: Option<String>,
  description: Option<String>,
}

#[utoipa::path(
  params(
    ("galaxy_id" = Uuid, Path, description = "id of the galaxy to update")
  ),
  request_body(
    content = UpdateGalaxyData,
    description = "data for updating the galaxy",
    content_type = "application/json"
  ),
  responses(
    (
      status = 200,
      description = "galaxy successfully updated",
      content_type = "application/json",
      body = Galaxy
    )
  )
)]
#[put("/galaxies/{galaxy_id}")]
pub async fn update_galaxy(
  path_param: Path<Uuid>,
  Json(data): Json<UpdateGalaxyData>,
) -> impl Responder {
  let id = path_param.into_inner();
  HttpResponse::Ok().body(format!("Galaxy {id} updated with {:?}", data))
}

#[utoipa::path(
  params(
    ("galaxy_id" = Uuid, Path, description = "id of galaxy to delete")
  ),
  responses(
    (
      status = 200,
      description = "galaxy successfully deleted",
      content_type = "application/json",
      body = Galaxy
    )
  )
)]
#[delete("/galaxies/{galaxy_id}")]
pub async fn delete_galaxy(path_param: Path<Uuid>) -> impl Responder {
  let id = path_param.into_inner();
  HttpResponse::Ok().body(format!("Galaxy {id} deleted"))
}
