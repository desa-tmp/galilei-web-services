use k8s_openapi::api::core::v1::Namespace;
use kube::{api::Request, Client, Error, Result};
use serde_json::{json, Value};
use uuid::Uuid;

#[derive(Clone)]
pub struct NamespaceApi {
  client: Client,
}

impl NamespaceApi {
  const BASE_PATH: &'static str = "/api/v1/namespaces";

  pub async fn try_default() -> Result<Self> {
    let client = Client::try_default().await?;

    Ok(Self { client })
  }

  pub async fn create(&self, galaxy_id: &Uuid) -> Result<Namespace> {
    let namespace = json!({
      "version": "v1",
      "metadata": {
        "name": galaxy_id,
        "labels": {
          "galaxy_id": galaxy_id
        }
      }
    });

    let namespace = serde_json::to_vec(&namespace).expect("Invalid namespace format");

    let req = Request::new(Self::BASE_PATH)
      .create(&Default::default(), namespace)
      .map_err(|err| Error::BuildRequest(err))?;

    Ok(self.client.request(req).await?)
  }

  pub async fn delete(&self, galaxy_id: &Uuid) -> Result<()> {
    let req = Request::new(Self::BASE_PATH)
      .delete(&galaxy_id.to_string(), &Default::default())
      .map_err(|err| Error::BuildRequest(err))?;

    let _: Value = self.client.request(req).await?;

    Ok(())
  }
}
