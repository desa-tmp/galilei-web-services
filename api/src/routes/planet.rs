use actix_web::{
  delete, get,
  http::StatusCode,
  post, put,
  web::{Json, Path, ServiceConfig},
};
use derive_more::From;
use serde::Serialize;
use validator::Validate;

use crate::database::Transaction;
use crate::error::{
  AlreadyExistsResponse, ApiResult, InternalErrorResponse, NotFoundResponse, ValidationResponse,
};
use crate::impl_json_responder;
use crate::models::planet::{
  CreatePlanetData, CrudOperations, GalaxyPath, Planet, PlanetPath, UpdatePlanetData,
};

#[derive(Serialize, From, utoipa::ToResponse)]
#[response(
  description = "all planets in the galaxy",
  content_type = "application/json"
)]
#[serde(transparent)]
pub struct PlanetsList(Vec<Planet>);
impl_json_responder!(PlanetsList, StatusCode::OK);

#[utoipa::path(
  params(GalaxyPath),
  responses(
    (status = OK, response = PlanetsList),
    (status = NOT_FOUND, response = NotFoundResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[get("/galaxies/{galaxy_id}/planets")]
pub async fn get_all_planets(
  mut tx: Transaction,
  path: Path<GalaxyPath>,
) -> ApiResult<PlanetsList> {
  let planets = Planet::all(&mut tx, &path).await?;

  Ok(PlanetsList::from(planets))
}

#[derive(Serialize, From, utoipa::ToResponse)]
#[response(
  description = "planet successfully created",
  content_type = "application/json"
)]
#[serde(transparent)]
pub struct PlanetCreated(Planet);
impl_json_responder!(PlanetCreated, StatusCode::CREATED);

#[utoipa::path(
  params(GalaxyPath),
  request_body(
    content = CreatePlanetData,
    description = "data for creating the planet",
    content_type = "application/json"
  ),
  responses(
    (status = OK, response = PlanetCreated),
    (status = NOT_FOUND, response = NotFoundResponse),
    (status = CONFLICT, response = AlreadyExistsResponse),
    (status = BAD_REQUEST, response = ValidationResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[post("/galaxies/{galaxy_id}/planets")]
pub async fn create_planet(
  mut tx: Transaction,
  path: Path<GalaxyPath>,
  Json(data): Json<CreatePlanetData>,
) -> ApiResult<PlanetCreated> {
  data.validate()?;

  let new_planet = Planet::create(&mut tx, &path, data).await?;

  Ok(PlanetCreated::from(new_planet))
}

#[derive(Serialize, From, utoipa::ToResponse)]
#[response(description = "specific planet", content_type = "application/json")]
pub struct SpecificPlanet(Planet);
impl_json_responder!(SpecificPlanet, StatusCode::OK);

#[utoipa::path(
  params(PlanetPath),
  responses(
    (status = OK, response = SpecificPlanet),
    (status = NOT_FOUND, response = NotFoundResponse),
    (status = CONFLICT, response = AlreadyExistsResponse),
    (status = BAD_REQUEST, response = ValidationResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[get("/galaxies/{galaxy_id}/planets/{planet_id}")]
pub async fn get_planet(mut tx: Transaction, path: Path<PlanetPath>) -> ApiResult<SpecificPlanet> {
  let planet = Planet::get(&mut tx, &path).await?;

  Ok(SpecificPlanet::from(planet))
}

#[derive(Serialize, From, utoipa::ToResponse)]
#[response(
  description = "planet successfully updated",
  content_type = "application/json"
)]
#[serde(transparent)]
pub struct PlanetUpdated(Planet);
impl_json_responder!(PlanetUpdated, StatusCode::OK);

#[utoipa::path(
  params(PlanetPath),
  request_body(
    content = UpdatePlanetData,
    description = "data for updating the planet",
    content_type = "application/json"
  ),
  responses(
    (status = OK, response = PlanetUpdated),
    (status = NOT_FOUND, response = NotFoundResponse),
    (status = CONFLICT, response = AlreadyExistsResponse),
    (status = BAD_REQUEST, response = ValidationResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[put("/galaxies/{galaxy_id}/planets/{planet_id}")]
pub async fn update_planet(
  mut tx: Transaction,
  path: Path<PlanetPath>,
  Json(data): Json<UpdatePlanetData>,
) -> ApiResult<PlanetUpdated> {
  data.validate()?;

  let updated_planet = Planet::update(&mut tx, &path, data).await?;

  Ok(PlanetUpdated::from(updated_planet))
}

#[derive(Serialize, From, utoipa::ToResponse)]
#[response(
  description = "planet successfully deleted",
  content_type = "application/json"
)]
#[serde(transparent)]
pub struct PlanetDeleted(Planet);
impl_json_responder!(PlanetDeleted, StatusCode::OK);

#[utoipa::path(
  params(PlanetPath),
  responses(
    (status = OK, response = PlanetDeleted),
    (status = NOT_FOUND, response = NotFoundResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[delete("/galaxies/{galaxy_id}/planets/{planet_id}")]
pub async fn delete_planet(
  mut tx: Transaction,
  path: Path<PlanetPath>,
) -> ApiResult<PlanetDeleted> {
  let deleted_planet = Planet::delete(&mut tx, &path).await?;

  Ok(PlanetDeleted::from(deleted_planet))
}

pub fn config(cfg: &mut ServiceConfig) {
  cfg
    .service(get_all_planets)
    .service(get_planet)
    .service(create_planet)
    .service(update_planet)
    .service(delete_planet);
}
