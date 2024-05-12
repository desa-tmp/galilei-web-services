use k8s_openapi::api::core::v1::Namespace;
use kube::{api::Request, Client, Error, Result};
use serde_json::{json, Value};

use crate::models::galaxy::Galaxy;

use super::ResourceBind;

const NAMESPACE_BASE_PATH: &'static str = "/api/v1/namespaces";

impl ResourceBind for Galaxy {
  type RequestResolver = Client;

  async fn create(&self, client: Self::RequestResolver) -> Result<()> {
    let namespace = json!({
      "version": "v1",
      "metadata": {
        "name": self.id,
        "labels": {
          "galaxy_id": self.id
        }
      }
    });

    let namespace = serde_json::to_vec(&namespace).expect("Invalid namespace format");

    let req = Request::new(NAMESPACE_BASE_PATH)
      .create(&Default::default(), namespace)
      .map_err(|err| Error::BuildRequest(err))?;

    let _: Namespace = client.request(req).await?;

    Ok(())
  }

  async fn update(&self, _client: Self::RequestResolver) -> Result<()> {
    unreachable!();
  }

  async fn delete(&self, client: Self::RequestResolver) -> Result<()> {
    let req = Request::new(NAMESPACE_BASE_PATH)
      .delete(&self.id.to_string(), &Default::default())
      .map_err(|err| Error::BuildRequest(err))?;

    let _: Value = client.request(req).await?;

    Ok(())
  }
}
