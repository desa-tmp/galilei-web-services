use actix_web::{
  delete, get,
  http::StatusCode,
  post, put,
  web::{Json, Path, Query, ServiceConfig},
  Either,
};
use actix_web_lab::sse::{self, Sse};
use derive_more::From;
use futures_util::{stream::Map, Stream, StreamExt};
use k8s_openapi::api::apps::v1::{Deployment, DeploymentStatus};
use kube::{
  runtime::{utils::EventFlatten, watcher, WatchStreamExt},
  Api, Client,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::impl_json_responder;
use crate::models::star::{
  CreateStarData, CrudOperations, GalaxyPath, Star, StarPath, UpdateStarData,
};
use crate::{database::Transaction, error::UnauthorizeResponse};
use crate::{
  error::{
    AlreadyExistsResponse, ApiResult, InternalErrorResponse, NotFoundResponse, ValidationResponse,
  },
  k8s::{ResourceBind, StarRequestResolver},
};

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

  let new_star = <Star as CrudOperations>::create(&mut tx, &path, &data).await?;

  ResourceBind::create(
    &new_star,
    StarRequestResolver::try_default(&new_star.galaxy_id).await?,
  )
  .await?;

  Ok(StarCreated::from(new_star))
}

#[derive(Deserialize, utoipa::IntoParams)]
pub struct WatchQuery {
  watch: Option<bool>,
}

#[derive(Serialize, utoipa::ToSchema)]
#[serde(tag = "status")]
pub enum StarStatus {
  Active,
  Failure,
}

#[derive(Serialize, From, utoipa::ToResponse)]
#[serde(untagged)]
#[response(description = "specific star in the galaxy")]
pub enum SpecificStar {
  Star(#[content("application/json")] Star),
  Status(#[content("text/event-stream")] StarStatus),
}
impl_json_responder!(SpecificStar, StatusCode::OK);

impl SpecificStar {
  fn status(status: DeploymentStatus) -> Self {
    // TODO fix this is valid only for deployments with replication set to 1
    Self::Status(match status.available_replicas {
      Some(_) => StarStatus::Active,
      None => StarStatus::Failure,
    })
  }
}

#[utoipa::path(
  params(StarPath, WatchQuery),
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
pub async fn get_star(
  mut tx: Transaction,
  path: Path<StarPath>,
  Query(WatchQuery { watch }): Query<WatchQuery>,
) -> ApiResult<
  Either<
    SpecificStar,
    Sse<
      Map<
        EventFlatten<
          impl Stream<Item = Result<watcher::Event<Deployment>, watcher::Error>> + Send,
          Deployment,
        >,
        impl FnMut(Result<Deployment, watcher::Error>) -> Result<sse::Event, watcher::Error>,
      >,
    >,
  >,
> {
  let star = Star::get(&mut tx, &path).await?;

  if !watch.unwrap_or(false) {
    return Ok(Either::Left(SpecificStar::from(star)));
  }

  let client = Client::try_default().await?;
  let api: Api<Deployment> = Api::namespaced(client, &format!("galaxy-{}", star.galaxy_id));
  let config = watcher::Config::default().labels(&format!("star_id={}", star.id));

  let stream = watcher(api, config).applied_objects().map(|deploy| {
    deploy.map(|deploy| {
      sse::Event::Data(
        sse::Data::new_json(SpecificStar::status(
          deploy.status.expect("missing status field from watcher"),
        ))
        .expect("Error serializing star status")
        .event("status"),
      )
    })
  });

  let sse_stream = Sse::from_stream(stream);

  Ok(Either::Right(sse_stream))
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

  let updated_star = <Star as CrudOperations>::update(&mut tx, &path, &data).await?;

  ResourceBind::update(
    &updated_star,
    StarRequestResolver::try_default(&updated_star.galaxy_id).await?,
  )
  .await?;

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
  let deleted_star = <Star as CrudOperations>::delete(&mut tx, &path).await?;

  ResourceBind::delete(
    &deleted_star,
    StarRequestResolver::try_default(&deleted_star.galaxy_id).await?,
  )
  .await?;

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
