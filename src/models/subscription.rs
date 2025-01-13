/*
 * SuperMQ Combined Service
 *
 * This is the SuperMQ Combined Service based on the OpenAPI 3.0 specification.  It is the HTTP API for managing SuperMQ. You can now help us improve the API whether it's by making changes to the definition itself or to the code. Some useful links: - [The SuperMQ repository](https://github.com/absmach/supermq) 
 *
 * The version of the OpenAPI document: 0.15.1
 * Contact: info@abstractmachines.fr
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Subscription {
    /// ULID id of the subscription.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// An id of the owner who created subscription.
    #[serde(rename = "owner_id", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<uuid::Uuid>,
    /// Topic to which the user subscribes.
    #[serde(rename = "topic", skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// The contact of the user to which the notification will be sent.
    #[serde(rename = "contact", skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
}

impl Subscription {
    pub fn new() -> Subscription {
        Subscription {
            id: None,
            owner_id: None,
            topic: None,
            contact: None,
        }
    }
}


