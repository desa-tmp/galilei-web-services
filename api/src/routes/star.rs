use super::FromPath;
use crate::error::{
  AlreadyExistsResponse, InternalErrorResponse, NotFoundResponse, ValidationResponse,
};
use crate::impl_json_responder;
use crate::models::star::{
  CreateStarData, CrudOperations, GalaxyPath, Star, StarPath, UpdateStarData,
};
use crate::prelude::*;
use actix_web::{
  delete, get,
  http::StatusCode,
  post, put,
  web::{Data, Json, Path, ServiceConfig},
};
use derive_more::From;
use serde::Serialize;
use validator::Validate;

impl FromPath for StarPath {}

#[derive(Serialize, From, utoipa::ToResponse)]
#[response(
  description = "all stars in the galaxy",
  content_type = "application/json"
)]
#[serde(transparent)]
pub struct StarsList(Vec<Star>);
impl_json_responder!(StarsList, StatusCode::OK);

#[utoipa::path(
  params(GalaxyPath),
  responses(
    (status = OK, response = StarsList),
    (status = NOT_FOUND, response = NotFoundResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[get("/galaxies/{galaxy_id}/stars")]
pub async fn get_all_stars(pool: Data<Pool>, path: Path<GalaxyPath>) -> Result<StarsList> {
  let galaxy_id = GalaxyPath::from_path(path);

  let stars = Star::all(&pool, galaxy_id).await?;

  Ok(StarsList::from(stars))
}

#[derive(Serialize, From, utoipa::ToResponse)]
#[response(
  description = "star successfully created",
  content_type = "application/json"
)]
#[serde(transparent)]
pub struct StarCreated(Star);
impl_json_responder!(StarCreated, StatusCode::CREATED);

#[utoipa::path(
  params(GalaxyPath),
  request_body(
    content = CreateStarData,
    description = "data for creating the star",
    content_type = "application/json"
  ),
  responses(
    (status = OK, response = StarCreated),
    (status = NOT_FOUND, response = NotFoundResponse),
    (status = CONFLICT, response = AlreadyExistsResponse),
    (status = BAD_REQUEST, response = ValidationResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[post("/galaxies/{galaxy_id}/stars")]
pub async fn create_star(
  pool: Data<Pool>,
  path: Path<GalaxyPath>,
  Json(data): Json<CreateStarData>,
) -> Result<StarCreated> {
  let galaxy_id = GalaxyPath::from_path(path);

  data.validate()?;

  let new_star = Star::create(&pool, galaxy_id, data).await?;

  Ok(StarCreated::from(new_star))
}

#[derive(Serialize, From, utoipa::ToResponse)]
#[response(
  description = "star successfully updated",
  content_type = "application/json"
)]
#[serde(transparent)]
pub struct StarUpdated(Star);
impl_json_responder!(StarUpdated, StatusCode::OK);

#[utoipa::path(
  params(StarPath),
  request_body(
    content = UpdateStarData,
    description = "data for updating the star",
    content_type = "application/json"
  ),
  responses(
    (status = OK, response = StarUpdated),
    (status = NOT_FOUND, response = NotFoundResponse),
    (status = CONFLICT, response = AlreadyExistsResponse),
    (status = BAD_REQUEST, response = ValidationResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[put("/galaxies/{galaxy_id}/stars/{star_id}")]
pub async fn update_star(
  pool: Data<Pool>,
  path: Path<StarPath>,
  Json(data): Json<UpdateStarData>,
) -> Result<StarUpdated> {
  let star_id = StarPath::from_path(path);

  data.validate()?;

  let updated_star = Star::update(&pool, star_id, data).await?;

  Ok(StarUpdated::from(updated_star))
}

#[derive(Serialize, From, utoipa::ToResponse)]
#[response(
  description = "star successfully deleted",
  content_type = "application/json"
)]
#[serde(transparent)]
pub struct StarDeleted(Star);
impl_json_responder!(StarDeleted, StatusCode::OK);

#[utoipa::path(
  params(StarPath),
  responses(
    (status = OK, response = StarDeleted),
    (status = NOT_FOUND, response = NotFoundResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[delete("/galaxies/{galaxy_id}/stars/{star_id}")]
pub async fn delete_star(pool: Data<Pool>, path: Path<StarPath>) -> Result<StarDeleted> {
  let star_id = StarPath::from_path(path);

  let deleted_star = Star::delete(&pool, star_id).await?;

  Ok(StarDeleted::from(deleted_star))
}

pub fn config(cfg: &mut ServiceConfig) {
  cfg
    .service(get_all_stars)
    .service(create_star)
    .service(update_star)
    .service(delete_star);
}