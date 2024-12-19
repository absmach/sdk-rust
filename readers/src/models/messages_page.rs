/*
 * SuperMQ reader service
 *
 * HTTP API for reading messages. Some useful links: - [The SuperMQ repository](https://github.com/absmach/supermq) 
 *
 * The version of the OpenAPI document: 0.15.1
 * Contact: info@abstractmachines.fr
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessagesPage {
    /// Total number of items that are present on the system.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    /// Number of items that were skipped during retrieval.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    /// Size of the subset that was retrieved.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<models::MessagesPageMessagesInner>>,
}

impl MessagesPage {
    pub fn new() -> MessagesPage {
        MessagesPage {
            total: None,
            offset: None,
            limit: None,
            messages: None,
        }
    }
}

