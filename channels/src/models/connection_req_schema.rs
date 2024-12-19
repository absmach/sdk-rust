/*
 * SuperMQ Channels Service
 *
 * This is the Channels Server based on the OpenAPI 3.0 specification.  It is the HTTP API for managing platform channels. You can now help us improve the API whether it's by making changes to the definition itself or to the code. Some useful links: - [The SuperMQ repository](https://github.com/absmach/supermq) 
 *
 * The version of the OpenAPI document: 0.15.1
 * Contact: info@abstractmachines.fr
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionReqSchema {
    /// Channel IDs.
    #[serde(rename = "channel_ids", skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<serde_json::Value>>,
    /// Client IDs
    #[serde(rename = "client_ids", skip_serializing_if = "Option::is_none")]
    pub client_ids: Option<Vec<serde_json::Value>>,
    /// Connection types.
    #[serde(rename = "types", skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<serde_json::Value>>,
}

impl ConnectionReqSchema {
    pub fn new() -> ConnectionReqSchema {
        ConnectionReqSchema {
            channel_ids: None,
            client_ids: None,
            types: None,
        }
    }
}

