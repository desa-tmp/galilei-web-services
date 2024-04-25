use actix_web::{
  delete, get,
  http::StatusCode,
  post, put,
  web::{Json, Path, ReqData, ServiceConfig},
};
use derive_more::From;
use serde::Serialize;

use validator::Validate;

use crate::database::Transaction;
use crate::error::{
  AlreadyExistsResponse, ApiResult, InternalErrorResponse, NotFoundResponse, ValidationResponse,
};
use crate::impl_json_responder;
use crate::models::{
  galaxy::{CreateGalaxyData, Galaxy, GalaxyPath, UpdateGalaxyData, UserId},
  CrudOperations,
};

use super::FromPath;

impl FromPath for GalaxyPath {}

#[derive(Serialize, From, utoipa::ToResponse)]
#[response(description = "all user galaxies", content_type = "application/json")]
#[serde(transparent)]
pub struct GalaxiesList(Vec<Galaxy>);
impl_json_responder!(GalaxiesList, StatusCode::OK);

#[utoipa::path(
  responses(
    (status = OK, response = GalaxiesList),
    (status = NOT_FOUND, response = NotFoundResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[get("/galaxies")]
pub async fn get_all_galaxies(
  mut tx: Transaction,
  user_id: ReqData<UserId>,
) -> ApiResult<GalaxiesList> {
  let galaxies = Galaxy::all(&mut tx, user_id.into_inner()).await?;

  Ok(GalaxiesList::from(galaxies))
}

#[derive(Serialize, From, utoipa::ToResponse)]
#[response(
  description = "galaxy successfully created",
  content_type = "application/json"
)]
#[serde(transparent)]
pub struct GalaxyCreated(Galaxy);
impl_json_responder!(GalaxyCreated, StatusCode::CREATED);

#[utoipa::path(
  request_body(
    content = CreateGalaxyData,
    description = "data for creating the galaxy",
    content_type = "application/json"
  ),
  responses(
    (status = OK, response = GalaxyCreated),
    (status = NOT_FOUND, response = NotFoundResponse),
    (status = CONFLICT, response = AlreadyExistsResponse),
    (status = BAD_REQUEST, response = ValidationResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[post("/galaxies")]
pub async fn create_galaxy(
  mut tx: Transaction,
  user_id: ReqData<UserId>,
  Json(data): Json<CreateGalaxyData>,
) -> ApiResult<GalaxyCreated> {
  data.validate()?;

  let new_galaxy = Galaxy::create(&mut tx, user_id.into_inner(), data).await?;

  Ok(GalaxyCreated::from(new_galaxy))
}

#[derive(Serialize, From, utoipa::ToResponse)]
#[response(
  description = "galaxy successfully updated",
  content_type = "application/json"
)]
#[serde(transparent)]
pub struct GalaxyUpdated(Galaxy);
impl_json_responder!(GalaxyUpdated, StatusCode::OK);

#[utoipa::path(
  params(GalaxyPath),
  request_body(
    content = UpdateGalaxyData,
    description = "data for updating the galaxy",
    content_type = "application/json"
  ),
  responses(
    (status = OK, response = GalaxyUpdated),
    (status = NOT_FOUND, response = NotFoundResponse),
    (status = CONFLICT, response = AlreadyExistsResponse),
    (status = BAD_REQUEST, response = ValidationResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[put("/galaxies/{galaxy_id}")]
pub async fn update_galaxy(
  mut tx: Transaction,
  path: Path<GalaxyPath>,
  Json(data): Json<UpdateGalaxyData>,
) -> ApiResult<GalaxyUpdated> {
  let galaxy_path = GalaxyPath::from_path(path);

  data.validate()?;

  let updated_galaxy = Galaxy::update(&mut tx, galaxy_path, data).await?;

  Ok(GalaxyUpdated::from(updated_galaxy))
}

#[derive(Serialize, From, utoipa::ToResponse)]
#[response(
  description = "galaxy successfully deleted",
  content_type = "application/json"
)]
#[serde(transparent)]
pub struct GalaxyDeleted(Galaxy);
impl_json_responder!(GalaxyDeleted, StatusCode::OK);

#[utoipa::path(
  params(GalaxyPath),
  responses(
    (status = OK, response = GalaxyDeleted),
    (status = NOT_FOUND, response = NotFoundResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[delete("/galaxies/{galaxy_id}")]
pub async fn delete_galaxy(
  mut tx: Transaction,
  path: Path<GalaxyPath>,
) -> ApiResult<GalaxyDeleted> {
  let galaxy_id = GalaxyPath::from_path(path);

  let deleted_galaxy = Galaxy::delete(&mut tx, galaxy_id).await?;

  Ok(GalaxyDeleted::from(deleted_galaxy))
}

pub fn config(cfg: &mut ServiceConfig) {
  cfg
    .service(get_all_galaxies)
    .service(create_galaxy)
    .service(update_galaxy)
    .service(delete_galaxy);
}
