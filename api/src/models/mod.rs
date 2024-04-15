use crate::database::Pool;
use async_trait::async_trait;

pub mod galaxy;
pub mod planet;
pub mod star;

#[async_trait]
pub trait CrudOperations: Sized {
  type OwnerIdent;
  type ResourceIdent;
  type CreateData;
  type UpdateData;

  async fn all(pool: &Pool, ident: Self::OwnerIdent) -> sqlx::Result<Vec<Self>>;
  async fn create(
    pool: &Pool,
    ident: Self::OwnerIdent,
    data: Self::CreateData,
  ) -> sqlx::Result<Self>;
  async fn update(
    pool: &Pool,
    ident: Self::ResourceIdent,
    data: Self::UpdateData,
  ) -> sqlx::Result<Self>;
  async fn delete(pool: &Pool, ident: Self::ResourceIdent) -> sqlx::Result<Self>;
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
