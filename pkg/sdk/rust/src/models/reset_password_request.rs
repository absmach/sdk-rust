/*
 * SuperMQ Users Service
 *
 * This is the Users Server based on the OpenAPI 3.0 specification.  It is the HTTP API for managing platform users. You can now help us improve the API whether it's by making changes to the definition itself or to the code. Some useful links: - [The SuperMQ repository](https://github.com/absmach/supermq) 
 *
 * The version of the OpenAPI document: 0.14.0
 * Contact: info@abstractmachines.fr
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    /// New password.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// New confirmation password.
    #[serde(rename = "confirm_password", skip_serializing_if = "Option::is_none")]
    pub confirm_password: Option<String>,
    /// Reset token generated and sent in email.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl ResetPasswordRequest {
    pub fn new() -> ResetPasswordRequest {
        ResetPasswordRequest {
            password: None,
            confirm_password: None,
            token: None,
        }
    }
}

