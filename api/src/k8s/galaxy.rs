use k8s_openapi::api::core::v1::Namespace;
use kube::{api::Request, Client, Error, Result};
use serde_json::{json, Value};

use crate::models::galaxy::Galaxy;

use super::ResourceBind;

const NAMESPACE_BASE_PATH: &'static str = "/api/v1/namespaces";

impl From<&Galaxy> for Namespace {
  fn from(value: &Galaxy) -> Self {
    let namespace = json!({
      "version": "v1",
      "metadata": {
        "name": format!("galaxy-{}", value.id),
        "labels": {
          "name": value.name,
          "galaxy_id": value.id
        }
      }
    });

    serde_json::from_value(namespace).expect("Invalid namespace format")
  }
}

impl ResourceBind for Galaxy {
  type RequestResolver = Client;

  async fn create(&self, client: Self::RequestResolver) -> Result<()> {
    let namespace = Namespace::from(self);
    let ns_buf = serde_json::to_vec(&namespace).expect("Error converting namespace to Vec<u8>");

    let req = Request::new(NAMESPACE_BASE_PATH)
      .create(&Default::default(), ns_buf)
      .map_err(|err| Error::BuildRequest(err))?;

    let _: Namespace = client.request(req).await?;

    Ok(())
  }

  async fn update(&self, client: Self::RequestResolver) -> Result<()> {
    let namespace = Namespace::from(self);
    let ns_buf = serde_json::to_vec(&namespace).expect("Error converting namespace to Vec<u8>");

    let req = Request::new(NAMESPACE_BASE_PATH)
      .replace(&format!("galaxy-{}", self.id), &Default::default(), ns_buf)
      .map_err(|err| Error::BuildRequest(err))?;

    let _: Value = client.request(req).await?;

    Ok(())
  }

  async fn delete(&self, client: Self::RequestResolver) -> Result<()> {
    let req = Request::new(NAMESPACE_BASE_PATH)
      .delete(&format!("galaxy-{}", self.id), &Default::default())
      .map_err(|err| Error::BuildRequest(err))?;

    let _: Value = client.request(req).await?;

    Ok(())
  }
}
