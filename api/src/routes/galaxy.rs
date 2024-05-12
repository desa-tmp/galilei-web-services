use actix_web::{
  delete, get,
  http::StatusCode,
  post, put,
  web::{Json, Path, ReqData, ServiceConfig},
};
use derive_more::From;
use kube::Client;
use serde::Serialize;
use validator::Validate;

use crate::impl_json_responder;
use crate::models::{
  galaxy::{CreateGalaxyData, Galaxy, GalaxyPath, UpdateGalaxyData, UserId},
  planet::Planet,
  star::Star,
  CrudOperations,
};
use crate::{database::Transaction, error::UnauthorizeResponse};
use crate::{
  error::{
    AlreadyExistsResponse, ApiResult, InternalErrorResponse, NotFoundResponse, ValidationResponse,
  },
  k8s::ResourceBind,
};

#[derive(Serialize, From, utoipa::ToResponse)]
#[response(description = "all user galaxies", content_type = "application/json")]
#[serde(transparent)]
pub struct GalaxiesList(Vec<Galaxy>);
impl_json_responder!(GalaxiesList, StatusCode::OK);

#[utoipa::path(
  responses(
    (status = OK, response = GalaxiesList),
    (status = NOT_FOUND, response = NotFoundResponse),
    (status = UNAUTHORIZED, response = UnauthorizeResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[get("/galaxies")]
pub async fn get_all_galaxies(
  mut tx: Transaction,
  user_id: ReqData<UserId>,
) -> ApiResult<GalaxiesList> {
  let galaxies = Galaxy::all(&mut tx, &user_id).await?;

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
    (status = UNAUTHORIZED, response = UnauthorizeResponse),
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

  let new_galaxy = <Galaxy as CrudOperations>::create(&mut tx, &user_id, &data).await?;

  ResourceBind::create(&new_galaxy, Client::try_default().await?).await?;

  Ok(GalaxyCreated::from(new_galaxy))
}

#[derive(Serialize, utoipa::ToResponse)]
#[response(
  description = "specific galaxy with all its stars and planets",
  content_type = "application/json"
)]
pub struct SpecificGalaxy {
  galaxy: Galaxy,
  stars: Vec<Star>,
  planets: Vec<Planet>,
}
impl_json_responder!(SpecificGalaxy, StatusCode::OK);

#[utoipa::path(
  params(GalaxyPath),
  responses(
    (status = OK, response = SpecificGalaxy),
    (status = NOT_FOUND, response = NotFoundResponse),
    (status = CONFLICT, response = AlreadyExistsResponse),
    (status = BAD_REQUEST, response = ValidationResponse),
    (status = UNAUTHORIZED, response = UnauthorizeResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[get("/galaxies/{galaxy_id}")]
pub async fn get_galaxy(mut tx: Transaction, path: Path<GalaxyPath>) -> ApiResult<SpecificGalaxy> {
  let galaxy = Galaxy::get(&mut tx, &path).await?;
  let stars = Star::all(&mut tx, &path).await?;
  let planets = Planet::all(&mut tx, &path).await?;

  Ok(SpecificGalaxy {
    galaxy,
    stars,
    planets,
  })
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
    (status = UNAUTHORIZED, response = UnauthorizeResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[put("/galaxies/{galaxy_id}")]
pub async fn update_galaxy(
  mut tx: Transaction,
  path: Path<GalaxyPath>,
  Json(data): Json<UpdateGalaxyData>,
) -> ApiResult<GalaxyUpdated> {
  data.validate()?;

  let updated_galaxy = <Galaxy as CrudOperations>::update(&mut tx, &path, &data).await?;

  // namespace name is galaxy_id not need to update

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
    (status = UNAUTHORIZED, response = UnauthorizeResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[delete("/galaxies/{galaxy_id}")]
pub async fn delete_galaxy(
  mut tx: Transaction,
  path: Path<GalaxyPath>,
) -> ApiResult<GalaxyDeleted> {
  let deleted_galaxy = <Galaxy as CrudOperations>::delete(&mut tx, &path).await?;

  ResourceBind::delete(&deleted_galaxy, Client::try_default().await?).await?;

  Ok(GalaxyDeleted::from(deleted_galaxy))
}

pub fn config(cfg: &mut ServiceConfig) {
  cfg
    .service(get_all_galaxies)
    .service(get_galaxy)
    .service(create_galaxy)
    .service(update_galaxy)
    .service(delete_galaxy);
}
