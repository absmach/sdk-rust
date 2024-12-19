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
pub struct ClientUpdate {
    /// Client name.
    #[serde(rename = "name")]
    pub name: String,
    /// Arbitrary, object-encoded client's data.
    #[serde(rename = "metadata")]
    pub metadata: serde_json::Value,
}

impl ClientUpdate {
    pub fn new(name: String, metadata: serde_json::Value) -> ClientUpdate {
        ClientUpdate {
            name,
            metadata,
        }
    }
}

