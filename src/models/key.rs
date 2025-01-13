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
pub struct Key {
    /// API key unique identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// In ID of the entity that issued the token.
    #[serde(rename = "issuer_id", skip_serializing_if = "Option::is_none")]
    pub issuer_id: Option<uuid::Uuid>,
    /// API key type. Keys of different type are processed differently.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// User's email or service identifier of API key subject.
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Time when the key is generated.
    #[serde(rename = "issued_at", skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<String>,
    /// Time when the Key expires. If this field is missing, that means that Key is valid indefinitely.
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl Key {
    pub fn new() -> Key {
        Key {
            id: None,
            issuer_id: None,
            r#type: None,
            subject: None,
            issued_at: None,
            expires_at: None,
        }
    }
}


