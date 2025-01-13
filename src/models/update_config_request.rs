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
pub struct UpdateConfigRequest {
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "name")]
    pub name: String,
}

impl UpdateConfigRequest {
    pub fn new(content: String, name: String) -> UpdateConfigRequest {
        UpdateConfigRequest {
            content,
            name,
        }
    }
}


