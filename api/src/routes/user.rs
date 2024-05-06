use actix_web::{
  get,
  web::{ReqData, ServiceConfig},
  HttpResponse,
};

use crate::{
  database::Transaction,
  error::ApiResult,
  models::{galaxy::UserId, user::User},
};

#[get("/users/me")]
pub async fn me(mut tx: Transaction, user_id: ReqData<UserId>) -> ApiResult<HttpResponse> {
  let me = User::get_by_id(&mut tx, &user_id).await?;

  Ok(HttpResponse::Ok().json(me))
}

pub fn config(cfg: &mut ServiceConfig) {
  cfg.service(me);
}
