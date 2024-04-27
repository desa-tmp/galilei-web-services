use sqlx::{Database, Pool, Transaction};
use std::{
  ops::{Deref, DerefMut},
  sync::Arc,
};

use crate::Error;

#[derive(Debug)]
pub struct LazyTx<DB: Database> {
  pool: Arc<Pool<DB>>,
  tx: Option<Transaction<'static, DB>>,
}

impl<DB: Database> LazyTx<DB> {
  pub fn new(pool: Arc<Pool<DB>>) -> Self {
    Self { pool, tx: None }
  }

  pub async fn begin(&mut self) -> Result<(), Error> {
    if self.tx.is_none() {
      self.tx = Some(self.pool.begin().await?);
    }

    Ok(())
  }

  pub fn tx(self) -> Option<Transaction<'static, DB>> {
    self.tx
  }
}

impl<DB: Database> Deref for LazyTx<DB> {
  type Target = Transaction<'static, DB>;

  fn deref(&self) -> &Self::Target {
    self
      .tx
      .as_ref()
      .expect("BUG: lazy transaction not initialized")
  }
}

impl<DB: Database> DerefMut for LazyTx<DB> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self
      .tx
      .as_mut()
      .expect("BUG: lazy transaction not initialized")
  }
}
