use async_trait::async_trait;

use crate::database::{DbError, DbResult, Operation, Pool};

pub mod galaxy;
pub mod planet;
pub mod session;
pub mod star;
pub mod user;

#[async_trait]
pub trait CrudOperations: Sized {
  type OwnerIdent: Send;
  type ResourceIdent: Send;
  type CreateData: Send;
  type UpdateData: Send;

  async fn all(_pool: &Pool, _ident: Self::OwnerIdent) -> DbResult<Vec<Self>> {
    Err(DbError::OperationNotImplemented(Operation::All))
  }

  async fn get(_pool: &Pool, _ident: Self::ResourceIdent) -> DbResult<Self> {
    Err(DbError::OperationNotImplemented(Operation::Get))
  }

  async fn create(
    _pool: &Pool,
    _ident: Self::OwnerIdent,
    _data: Self::CreateData,
  ) -> DbResult<Self> {
    Err(DbError::OperationNotImplemented(Operation::Create))
  }

  async fn update(
    _pool: &Pool,
    _ident: Self::ResourceIdent,
    _data: Self::UpdateData,
  ) -> DbResult<Self> {
    Err(DbError::OperationNotImplemented(Operation::Update))
  }

  async fn delete(_pool: &Pool, _ident: Self::ResourceIdent) -> DbResult<Self> {
    Err(DbError::OperationNotImplemented(Operation::Delete))
  }
}

#[macro_export]
macro_rules! gen_update_data {
    (
      $update_struct_name:ident,
      $(#[$meta:meta])*
      $vis:vis struct $struct_name:ident {
        $(
          $(#[$field_meta:meta])*
          $field_vis:vis $field_name:ident : $field_type:ty
        ),*$(,)+
      }
    ) => {
      $(#[$meta])*
      $vis struct $struct_name {
        $(
          $(#[$field_meta])*
          $field_vis $field_name : $field_type
        ),*
      }

      $(#[$meta])*
      $vis struct $update_struct_name {
        $(
          $(#[$field_meta])*
          $field_vis $field_name : Option<$field_type>
        ),*
      }
    };
}
