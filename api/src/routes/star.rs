use actix_web::{
  delete, get,
  http::StatusCode,
  post, put,
  web::{Json, Path, ServiceConfig},
};
use derive_more::From;
use serde::Serialize;
use validator::Validate;

use crate::error::{
  AlreadyExistsResponse, ApiResult, InternalErrorResponse, NotFoundResponse, ValidationResponse,
};
use crate::impl_json_responder;
use crate::models::star::{
  CreateStarData, CrudOperations, GalaxyPath, Star, StarPath, UpdateStarData,
};
use crate::{database::Transaction, error::UnauthorizeResponse};

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
    (status = UNAUTHORIZED, response = UnauthorizeResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[get("/galaxies/{galaxy_id}/stars")]
pub async fn get_all_stars(mut tx: Transaction, path: Path<GalaxyPath>) -> ApiResult<StarsList> {
  let stars = Star::all(&mut tx, &path).await?;

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
    (status = UNAUTHORIZED, response = UnauthorizeResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[post("/galaxies/{galaxy_id}/stars")]
pub async fn create_star(
  mut tx: Transaction,
  path: Path<GalaxyPath>,
  Json(data): Json<CreateStarData>,
) -> ApiResult<StarCreated> {
  data.validate()?;

  let new_star = Star::create(&mut tx, &path, data).await?;

  Ok(StarCreated::from(new_star))
}

#[derive(Serialize, From, utoipa::ToResponse)]
#[response(description = "specific star", content_type = "application/json")]
pub struct SpecificStar(Star);
impl_json_responder!(SpecificStar, StatusCode::OK);

#[utoipa::path(
  params(StarPath),
  responses(
    (status = OK, response = SpecificStar),
    (status = NOT_FOUND, response = NotFoundResponse),
    (status = CONFLICT, response = AlreadyExistsResponse),
    (status = BAD_REQUEST, response = ValidationResponse),
    (status = UNAUTHORIZED, response = UnauthorizeResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[get("/galaxies/{galaxy_id}/stars/{star_id}")]
pub async fn get_star(mut tx: Transaction, path: Path<StarPath>) -> ApiResult<SpecificStar> {
  let star = Star::get(&mut tx, &path).await?;

  Ok(SpecificStar::from(star))
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
    (status = UNAUTHORIZED, response = UnauthorizeResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[put("/galaxies/{galaxy_id}/stars/{star_id}")]
pub async fn update_star(
  mut tx: Transaction,
  path: Path<StarPath>,
  Json(data): Json<UpdateStarData>,
) -> ApiResult<StarUpdated> {
  data.validate()?;

  let updated_star = Star::update(&mut tx, &path, data).await?;

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
    (status = UNAUTHORIZED, response = UnauthorizeResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[delete("/galaxies/{galaxy_id}/stars/{star_id}")]
pub async fn delete_star(mut tx: Transaction, path: Path<StarPath>) -> ApiResult<StarDeleted> {
  let deleted_star = Star::delete(&mut tx, &path).await?;

  Ok(StarDeleted::from(deleted_star))
}

pub fn config(cfg: &mut ServiceConfig) {
  cfg
    .service(get_all_stars)
    .service(get_star)
    .service(create_star)
    .service(update_star)
    .service(delete_star);
}
