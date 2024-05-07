use actix_web::{
  get,
  http::StatusCode,
  web::{ReqData, ServiceConfig},
  HttpResponse,
};
use derive_more::From;
use serde::Serialize;
use utoipa::ToResponse;

use crate::{
  database::Transaction,
  error::{ApiResult, InternalErrorResponse, UnauthorizeResponse},
  impl_json_responder,
  models::{galaxy::UserId, user::User},
};

#[derive(Serialize, From, ToResponse)]
#[serde(transparent)]
pub struct UserResponse(User);
impl_json_responder!(UserResponse, StatusCode::OK);

#[utoipa::path(
  responses(
    (status = OK, response = UserResponse),
    (status = UNAUTHORIZED, response = UnauthorizeResponse),
    (status = INTERNAL_SERVER_ERROR, response = InternalErrorResponse)
  )
)]
#[get("/users/me")]
pub async fn me(mut tx: Transaction, user_id: ReqData<UserId>) -> ApiResult<HttpResponse> {
  let me = User::get_by_id(&mut tx, &user_id).await?;

  Ok(HttpResponse::Ok().json(me))
}

pub fn config(cfg: &mut ServiceConfig) {
  cfg.service(me);
}
