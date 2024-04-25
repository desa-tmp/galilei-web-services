use actix_web::{
  body::BoxBody,
  dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
  HttpMessage, ResponseError,
};
use log::debug;
use sqlx::{Database, Pool};
use std::{
  cell::RefCell,
  future::{ready, Future, Ready},
  marker::PhantomData,
  pin::Pin,
  rc::Rc,
  sync::Arc,
};

use crate::slot::{Slot, SlotState};
use crate::{error::Error, lazy::LazyTx};

#[derive(Debug)]
pub struct TransactionService<DB: Database, E = Error> {
  pool: Arc<Pool<DB>>,
  _error: PhantomData<E>,
}

impl<DB: Database, E> TransactionService<DB, E> {
  pub fn new(pool: Arc<Pool<DB>>) -> Self {
    Self {
      pool: Arc::clone(&pool),
      _error: PhantomData,
    }
  }
}

impl<S, DB, E> Transform<S, ServiceRequest> for TransactionService<DB, E>
where
  S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = actix_web::Error>
    + 'static,
  S::Future: 'static,
  DB: Database,
  E: From<Error> + ResponseError + 'static,
{
  type Response = ServiceResponse<BoxBody>;
  type Error = actix_web::Error;
  type InitError = ();
  type Transform = TransactionMiddleware<S, DB, E>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(TransactionMiddleware {
      service: Rc::new(RefCell::new(service)),
      pool: Arc::clone(&self.pool),
      _error: PhantomData,
    }))
  }
}

pub struct TransactionMiddleware<S, DB: Database, E> {
  service: Rc<RefCell<S>>,
  pool: Arc<Pool<DB>>,
  _error: PhantomData<E>,
}

impl<S, DB, E> Service<ServiceRequest> for TransactionMiddleware<S, DB, E>
where
  S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = actix_web::Error>
    + 'static,
  S::Future: 'static,
  DB: Database,
  E: From<Error> + ResponseError + 'static,
{
  type Response = ServiceResponse<BoxBody>;
  type Error = actix_web::Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

  forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let svc = Rc::clone(&self.service);
    let pool = Arc::clone(&self.pool);

    Box::pin(async move {
      let slot = Slot::new(LazyTx::new(pool));

      if let Some(slot) = req.extensions_mut().insert(Slot::clone(&slot)) {
        debug!("slot {:#?} replaced", slot);
      }

      let response = svc.call(req).await.map(|res| res.map_into_boxed_body());

      let tx = match slot.steal() {
        SlotState::Value(v) => v.tx(),
        _ => Err(E::from(Error::MultipleExtractors))?,
      };

      let tx_res = match response {
        Ok(ref res) => {
          let status_code = res.status();
          if status_code.is_client_error() || status_code.is_server_error() {
            tx.rollback().await
          } else {
            tx.commit().await
          }
        }
        Err(_) => tx.rollback().await,
      };

      tx_res.map_err(|err| E::from(Error::Database(err)))?;

      response
    })
  }
}
