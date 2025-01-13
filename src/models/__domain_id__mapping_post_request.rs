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
pub struct DomainIdMappingPostRequest {
    #[serde(rename = "external_id")]
    pub external_id: String,
    #[serde(rename = "external_key")]
    pub external_key: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DomainIdMappingPostRequest {
    pub fn new(external_id: String, external_key: String) -> DomainIdMappingPostRequest {
        DomainIdMappingPostRequest {
            external_id,
            external_key,
            name: None,
        }
    }
}


