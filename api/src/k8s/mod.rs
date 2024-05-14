use kube::Result;

mod galaxy;
mod planet;
mod star;

pub use star::StarRequestResolver;
pub use planet::PlanetRequestResolver;

pub trait ResourceBind: Sized {
  type RequestResolver;

  async fn create(&self, api: Self::RequestResolver) -> Result<()>;

  async fn update(&self, api: Self::RequestResolver) -> Result<()>;

  async fn delete(&self, api: Self::RequestResolver) -> Result<()>;
}
