/*
 * SuperMQ Groups Service
 *
 * This is the Groups Server based on the OpenAPI 3.0 specification.  It is the HTTP API for managing platform groups. You can now help us improve the API whether it's by making changes to the definition itself or to the code. Some useful links: - [The SuperMQ repository](https://github.com/absmach/supermq) 
 *
 * The version of the OpenAPI document: 0.15.1
 * Contact: info@abstractmachines.fr
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateRoleObj {
    /// Role's name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateRoleObj {
    pub fn new() -> UpdateRoleObj {
        UpdateRoleObj {
            name: None,
        }
    }
}

