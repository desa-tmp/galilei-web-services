use std::{
  cell::RefCell,
  future::{ready, Future, Ready},
  pin::Pin,
  rc::Rc,
  sync::Arc,
};

use actix_web::{
  body::BoxBody,
  dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
  Error, HttpMessage,
};
use log::debug;

use crate::{auth::Token, database::Pool, error::ApiError, models::session::Session};

pub struct AuthService {
  pool: Arc<Pool>,
}

impl AuthService {
  pub fn new(pool: Arc<Pool>) -> Self {
    Self { pool }
  }
}

impl<S> Transform<S, ServiceRequest> for AuthService
where
  S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
  S::Future: 'static,
{
  type Response = ServiceResponse<BoxBody>;
  type Error = Error;
  type InitError = ();
  type Transform = AuthMiddleware<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(AuthMiddleware {
      service: Rc::new(RefCell::new(service)),
      pool: self.pool.clone(),
    }))
  }
}

pub struct AuthMiddleware<S> {
  service: Rc<RefCell<S>>,
  pool: Arc<Pool>,
}

const SESSION_COOKIE: &str = "session";

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
  S::Future: 'static,
{
  type Response = ServiceResponse<BoxBody>;
  type Error = Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

  forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let pool = self.pool.clone();
    let svc = self.service.clone();

    Box::pin(async move {
      let session_cookie = req.cookie(SESSION_COOKIE).ok_or_else(|| {
        debug!("Session cookie not found");
        Error::from(ApiError::Unauthorize)
      })?;

      let token = Token::new(session_cookie.value().to_string());

      let mut conn = pool.acquire().await.map_err(|err| {
        debug!("Error creating connection {}", err);
        Error::from(ApiError::Unauthorize)
      })?;

      let user_id = Session::verify_token(&mut conn, token).await.map_err(|_| {
        debug!("Invalid token");
        Error::from(ApiError::Unauthorize)
      })?;

      if let Some(_) = req.extensions_mut().insert(user_id) {
        debug!("user id already exists");
        return Err(Error::from(ApiError::InternalError));
      }

      svc.call(req).await.map(|res| res.map_into_boxed_body())
    })
  }
}
