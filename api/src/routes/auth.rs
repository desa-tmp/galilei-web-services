use actix_web::{
  cookie::{time::OffsetDateTime, Cookie},
  post,
  web::{Json, ServiceConfig},
  HttpResponse, Responder,
};
use chrono::{Days, NaiveDateTime, Utc};
use serde::Deserialize;
use validator::Validate;

use crate::auth::Token;
use crate::database::{DbResult, Transaction};
use crate::error::ApiResult;
use crate::models::{
  session::Session,
  user::{Credentials, User},
};

#[derive(Debug)]
struct AuthResponse {
  user: User,
  expires: Option<OffsetDateTime>,
  token: Token,
}

impl AuthResponse {
  async fn session(mut tx: Transaction, remember: bool, user: User) -> DbResult<Self> {
    let token = Token::generate()?;

    let now = Utc::now();

    let expires = if remember {
      Some(
        NaiveDateTime::new(now.date_naive(), now.time())
          .checked_add_days(Days::new(1))
          .expect("Out of range"),
      )
    } else {
      None
    };

    let session = Session::create(&mut tx, &token, expires, &user.id).await?;

    let expires = session.expires.map(|timestamp| {
      let utc = timestamp.and_utc();
      // this is just a date-time conversion between two different types from two different libraries
      // so this function should never cause errors
      OffsetDateTime::from_unix_timestamp(utc.timestamp()).expect("invalid timestamp")
    });

    Ok(Self {
      user,
      expires,
      token,
    })
  }
}

impl Responder for AuthResponse {
  type Body = actix_web::body::BoxBody;

  fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
    let AuthResponse {
      user,
      expires,
      token,
    } = self;

    let session_cookie = Cookie::build("session", token.value())
      .expires(expires)
      .http_only(true)
      .finish();

    HttpResponse::Ok().cookie(session_cookie).json(user)
  }
}

#[derive(Debug, Deserialize, Validate)]
struct AuthData {
  #[serde(flatten)]
  #[validate(nested)]
  credentials: Credentials,
  remember: bool,
}

#[post("/register")]
pub async fn register(
  mut tx: Transaction,
  Json(auth_data): Json<AuthData>,
) -> ApiResult<AuthResponse> {
  auth_data.validate()?;

  let AuthData {
    credentials,
    remember,
  } = auth_data;

  let new_user = User::create(&mut tx, credentials).await?;

  Ok(AuthResponse::session(tx, remember, new_user).await?)
}

#[post("/login")]
pub async fn login(
  mut tx: Transaction,
  Json(auth_data): Json<AuthData>,
) -> ApiResult<AuthResponse> {
  // validate only the received data not the auth_data in database
  auth_data.validate()?;

  let AuthData {
    credentials,
    remember,
  } = auth_data;

  let user = User::verify_credentials(&mut tx, credentials).await?;

  Ok(AuthResponse::session(tx, remember, user).await?)
}

pub fn config(cfg: &mut ServiceConfig) {
  cfg.service(register).service(login);
}
