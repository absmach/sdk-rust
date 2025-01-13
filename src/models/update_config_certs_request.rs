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
pub struct UpdateConfigCertsRequest {
    #[serde(rename = "client_cert", skip_serializing_if = "Option::is_none")]
    pub client_cert: Option<String>,
    #[serde(rename = "client_key", skip_serializing_if = "Option::is_none")]
    pub client_key: Option<String>,
    #[serde(rename = "ca_cert", skip_serializing_if = "Option::is_none")]
    pub ca_cert: Option<String>,
}

impl UpdateConfigCertsRequest {
    pub fn new() -> UpdateConfigCertsRequest {
        UpdateConfigCertsRequest {
            client_cert: None,
            client_key: None,
            ca_cert: None,
        }
    }
}


