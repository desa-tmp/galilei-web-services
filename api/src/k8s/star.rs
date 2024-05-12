use k8s_openapi::api::{apps::v1::Deployment, core::v1::Service, networking::v1::Ingress};
use kube::{Api, Client, Result};
use serde_json::json;

use crate::models::star::Star;

use super::ResourceBind;

pub struct StarRequestResolver {
  deploy: Api<Deployment>,
  svc: Api<Service>,
  ingress: Api<Ingress>,
}

impl StarRequestResolver {
  pub async fn try_default(ns: &str) -> Result<Self> {
    let client = Client::try_default().await?;

    Ok(Self {
      deploy: Api::namespaced(client.clone(), ns),
      svc: Api::namespaced(client.clone(), ns),
      ingress: Api::namespaced(client, ns),
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
        "name": value.id,
        "namespace": value.galaxy_id,
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
                "name": value.id,
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
        "name": value.id,
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
        "name": value.id,
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
                      "name": value.id,
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
    let k8s_name = self.id.to_string();

    let _ = api
      .deploy
      .replace(&k8s_name, &Default::default(), &Deployment::from(self))
      .await?;

    let _ = api
      .svc
      .replace(&k8s_name, &Default::default(), &Service::from(self))
      .await?;

    let _ = api
      .ingress
      .replace(&k8s_name, &Default::default(), &Ingress::from(self))
      .await?;

    Ok(())
  }

  async fn delete(&self, api: Self::RequestResolver) -> Result<()> {
    let _ = api.deploy.delete(&self.name, &Default::default()).await?;

    let _ = api.svc.delete(&self.name, &Default::default()).await?;

    let _ = api.ingress.delete(&self.name, &Default::default()).await?;

    Ok(())
  }
}
