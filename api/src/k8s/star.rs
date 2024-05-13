use k8s_openapi::api::{apps::v1::Deployment, core::v1::Service, networking::v1::Ingress};
use kube::{
  api::{DeleteParams, PostParams},
  Api, Client, Result,
};
use serde_json::json;
use uuid::Uuid;

use crate::models::star::Star;

use super::ResourceBind;

pub struct StarRequestResolver {
  deploy: Api<Deployment>,
  svc: Api<Service>,
  ingress: Api<Ingress>,
}

impl StarRequestResolver {
  pub async fn try_default(galaxy_id: &Uuid) -> Result<Self> {
    let client = Client::try_default().await?;
    let galaxy_ns = format!("galaxy-{}", galaxy_id);

    Ok(Self {
      deploy: Api::namespaced(client.clone(), &galaxy_ns),
      svc: Api::namespaced(client.clone(), &galaxy_ns),
      ingress: Api::namespaced(client, &galaxy_ns),
    })
  }
}

const PORT: i32 = 80;

impl From<&Star> for Deployment {
  fn from(value: &Star) -> Self {
    let deployment = json!({
      "apiVersion": "apps/v1",
      "kind": "Deployment",
      "metadata": {
        "name": format!("star-{}", value.id),
        "namespace": format!("galaxy-{}", value.galaxy_id),
        "labels": {
          "star_name": value.name,
          "star_id": value.id,
          "galaxy_id": value.galaxy_id,
        },
      },
      "spec": {
        "replicas": 2,
        "selector": {
          "matchLabels": {
            "star_id": value.id,
          },
        },
        "template": {
          "metadata": {
            "labels": {
              "star_name": value.name,
              "star_id": value.id,
            },
          },
          "spec": {
            "containers": [
              {
                "name": format!("star-{}", value.id),
                "image": value.nebula.to_lowercase(),
                "env": [
                  {
                    "name": "ADDRESS",
                    "value": "0.0.0.0"
                  },
                  {
                    "name": "PORT",
                    "value": PORT.to_string()
                  }
                ],
                "ports": [
                  {
                    "containerPort": PORT
                  }
                ]
              }
            ],
          }
        },
      },
    });

    serde_json::from_value(deployment).expect("Invalid deployment")
  }
}

impl From<&Star> for Service {
  fn from(value: &Star) -> Self {
    let svc = json!({
      "apiVersion": "v1",
      "kind": "Service",
      "metadata": {
        "name": format!("star-{}", value.id),
        "namespace": format!("galaxy-{}", value.galaxy_id),
        "labels": {
          "star_name": value.name,
          "star_id": value.id,
          "galaxy_id": value.galaxy_id,
        },
      },
      "spec": {
        "selector": {
          "star_id": value.id,
        },
        "ports": [
          {
            "port": PORT,
            "targetPort": PORT,
          },
        ],
      },
    });

    serde_json::from_value(svc).expect("Invalid service")
  }
}

impl From<&Star> for Ingress {
  fn from(value: &Star) -> Self {
    let ingress = json!({
      "apiVersion": "networking.k8s.io/v1",
      "kind": "Ingress",
      "metadata": {
        "name": format!("star-{}", value.id),
        "namespace": format!("galaxy-{}", value.galaxy_id),
        "labels": {
          "star_name": value.name,
          "star_id": value.id,
          "galaxy_id": value.galaxy_id,
        },
        "annotations": {
          "ingress.kubernetes.io/ssl-redirect": "false"
        }
      },
      "spec": {
        "rules": [
          {
            "host": format!("{}.localhost", value.name),
            "http": {
              "paths": [
                {
                  "path": "/",
                  "pathType": "Prefix",
                  "backend": {
                    "service": {
                      "name": format!("star-{}", value.id),
                      "port": {
                        "number": PORT
                      }
                    }
                  }
                }
              ]
            }
          }
        ]
      }
    });

    serde_json::from_value(ingress).expect("Invalid Ingress")
  }
}

impl ResourceBind for Star {
  type RequestResolver = StarRequestResolver;

  async fn create(&self, api: Self::RequestResolver) -> Result<()> {
    let _ = api
      .deploy
      .create(&Default::default(), &Deployment::from(self))
      .await?;

    let _ = api
      .svc
      .create(&Default::default(), &Service::from(self))
      .await?;

    let _ = api
      .ingress
      .create(&Default::default(), &Ingress::from(self))
      .await?;

    Ok(())
  }

  async fn update(&self, api: Self::RequestResolver) -> Result<()> {
    let k8s_name = format!("star-{}", self.id.to_string());
    let pp = PostParams::default();

    let _ = api
      .deploy
      .replace(&k8s_name, &pp, &Deployment::from(self))
      .await?;

    let _ = api
      .svc
      .replace(&k8s_name, &pp, &Service::from(self))
      .await?;

    let _ = api
      .ingress
      .replace(&k8s_name, &pp, &Ingress::from(self))
      .await?;

    Ok(())
  }

  async fn delete(&self, api: Self::RequestResolver) -> Result<()> {
    let dp = DeleteParams::default();

    let _ = api.deploy.delete(&self.name, &dp).await?;

    let _ = api.svc.delete(&self.name, &dp).await?;

    let _ = api.ingress.delete(&self.name, &dp).await?;

    Ok(())
  }
}
