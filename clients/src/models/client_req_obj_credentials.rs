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
pub struct ClientReqObjCredentials {
    /// Client's identity will be used as its unique identifier
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// Free-form account secret used for acquiring auth token(s).
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl ClientReqObjCredentials {
    pub fn new() -> ClientReqObjCredentials {
        ClientReqObjCredentials {
            identity: None,
            secret: None,
        }
    }
}

