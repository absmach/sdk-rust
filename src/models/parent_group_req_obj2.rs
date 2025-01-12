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
pub struct ParentGroupReqObj2 {
    /// Parent group unique identifier.
    #[serde(rename = "group_id")]
    pub group_id: uuid::Uuid,
}

impl ParentGroupReqObj2 {
    pub fn new(group_id: uuid::Uuid) -> ParentGroupReqObj2 {
        ParentGroupReqObj2 {
            group_id,
        }
    }
}


