/*
 * SuperMQ Clients Service
 *
 * This is the Clients Server based on the OpenAPI 3.0 specification.  It is the HTTP API for managing platform clients. You can now help us improve the API whether it's by making changes to the definition itself or to the code. Some useful links: - [The SuperMQ repository](https://github.com/absmach/supermq) 
 *
 * The version of the OpenAPI document: 0.15.1
 * Contact: info@abstractmachines.fr
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientWithEmptySecret {
    /// Client unique identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// Client name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Client tags.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// ID of the domain to which client belongs.
    #[serde(rename = "domain_id", skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<uuid::Uuid>,
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Box<models::ClientWithEmptySecretCredentials>>,
    /// Arbitrary, object-encoded client's data.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Client Status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Time when the channel was created.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Time when the channel was created.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl ClientWithEmptySecret {
    pub fn new() -> ClientWithEmptySecret {
        ClientWithEmptySecret {
            id: None,
            name: None,
            tags: None,
            domain_id: None,
            credentials: None,
            metadata: None,
            status: None,
            created_at: None,
            updated_at: None,
        }
    }
}

