/*
 * Magistrala Domains Service
 *
 * This is the Domains Server based on the OpenAPI 3.0 specification.  It is the HTTP API for managing platform domains. You can now help us improve the API whether it's by making changes to the definition itself or to the code. Some useful links: - [The Magistrala repository](https://github.com/absmach/magistrala) 
 *
 * The version of the OpenAPI document: 0.15.1
 * Contact: info@abstractmachines.fr
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainsPage {
    #[serde(rename = "domains")]
    pub domains: Vec<models::Domain>,
    /// Total number of items.
    #[serde(rename = "total")]
    pub total: i32,
    /// Number of items to skip during retrieval.
    #[serde(rename = "offset")]
    pub offset: i32,
    /// Maximum number of items to return in one page.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl DomainsPage {
    pub fn new(domains: Vec<models::Domain>, total: i32, offset: i32) -> DomainsPage {
        DomainsPage {
            domains,
            total,
            offset,
            limit: None,
        }
    }
}

