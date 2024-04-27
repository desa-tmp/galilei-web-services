use actix_web::{dev::Payload, FromRequest, HttpMessage, HttpRequest, ResponseError};
use sqlx::Database;
use std::future::Future;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;

use crate::error::Error;
use crate::lazy::LazyTx;
use crate::slot::{Slot, SlotRef, SlotState};

type Tx<DB> = sqlx::Transaction<'static, DB>;

pub struct Transaction<DB: Database, E = Error>(SlotRef<LazyTx<DB>>, PhantomData<E>);

impl<DB, E> Transaction<DB, E>
where
  DB: Database,
  E: From<Error>,
{
  async fn create(mut tx: SlotRef<LazyTx<DB>>) -> Result<Self, E> {
    LazyTx::<DB>::begin(&mut tx).await?;

    Ok(Self(tx, PhantomData::<E>))
  }

  fn take(self) -> Result<Tx<DB>, E> {
    match self.0.steal() {
      SlotState::Value(v) => Ok(v.tx().expect("BUG: transaction not initialized")),
      _ => Err(E::from(Error::MultipleExtractors)),
    }
  }

  pub async fn commit(self) -> Result<(), E> {
    self.take()?.commit().await.map_err(Error::from)?;

    Ok(())
  }

  pub async fn rollback(self) -> Result<(), E> {
    self.take()?.rollback().await.map_err(Error::from)?;

    Ok(())
  }
}

impl<DB: Database, E> Deref for Transaction<DB, E> {
  type Target = DB::Connection;

  fn deref(&self) -> &Self::Target {
    self.0.as_deref()
  }
}

impl<DB: Database, E> DerefMut for Transaction<DB, E> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.0.as_deref_mut()
  }
}

impl<DB, E> FromRequest for Transaction<DB, E>
where
  DB: Database,
  E: From<Error> + ResponseError + 'static,
{
  type Error = E;
  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

  fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
    let req = req.clone();

    Box::pin(async move {
      let slot = req
        .extensions_mut()
        .remove::<Slot<LazyTx<DB>>>()
        .ok_or(E::from(Error::MissingMiddleware))?;

      match slot.take() {
        SlotState::Value(v) => Ok(Self::create(v).await?),
        _ => Err(E::from(Error::MultipleExtractors)),
      }
    })
  }
}
