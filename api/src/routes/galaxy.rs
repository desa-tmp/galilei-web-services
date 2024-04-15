use super::FromPath;
use crate::error::{
  AlreadyExistsResponse, InternalErrorResponse, NotFoundResponse, ValidationResponse,
};
use crate::impl_json_responder;
use crate::models::galaxy::{CreateGalaxyData, Galaxy, GalaxyPath, UpdateGalaxyData, UserId};
use crate::models::CrudOperations;
use crate::prelude::*;
use actix_web::{
  delete, get,
  http::StatusCode,
  post, put,
  web::{Data, Json, Path, ServiceConfig},
};
use derive_more::From;
use serde::Serialize;
use std::str::FromStr;
use validator::Validate;

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
pub async fn get_all_galaxies(pool: Data<Pool>) -> Result<GalaxiesList> {
  let user_id = UserId::from_str("4cde5822-0abb-41f3-abad-4b08e01fcbc3").expect("Valid UUID");

  let galaxies = Galaxy::all(&pool, user_id).await?;

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
  pool: Data<Pool>,
  Json(data): Json<CreateGalaxyData>,
) -> Result<GalaxyCreated> {
  let user_id = UserId::from_str("4cde5822-0abb-41f3-abad-4b08e01fcbc3").expect("Valid UUID");

  data.validate()?;

  let new_galaxy = Galaxy::create(&pool, user_id, data).await?;

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
  pool: Data<Pool>,
  path: Path<GalaxyPath>,
  Json(data): Json<UpdateGalaxyData>,
) -> Result<GalaxyUpdated> {
  let galaxy_path = GalaxyPath::from_path(path);

  data.validate()?;

  let updated_galaxy = Galaxy::update(&pool, galaxy_path, data).await?;

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
pub async fn delete_galaxy(pool: Data<Pool>, path: Path<GalaxyPath>) -> Result<GalaxyDeleted> {
  let galaxy_id = GalaxyPath::from_path(path);

  let deleted_galaxy = Galaxy::delete(&pool, galaxy_id).await?;

  Ok(GalaxyDeleted::from(deleted_galaxy))
}

pub fn config(cfg: &mut ServiceConfig) {
  cfg
    .service(get_all_galaxies)
    .service(create_galaxy)
    .service(update_galaxy)
    .service(delete_galaxy);
}
