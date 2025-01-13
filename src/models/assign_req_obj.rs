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
pub struct AssignReqObj {
    /// Members IDs
    #[serde(rename = "members")]
    pub members: Vec<String>,
    /// Permission relations.
    #[serde(rename = "relation")]
    pub relation: String,
    /// Member kind.
    #[serde(rename = "member_kind")]
    pub member_kind: String,
}

impl AssignReqObj {
    pub fn new(members: Vec<String>, relation: String, member_kind: String) -> AssignReqObj {
        AssignReqObj {
            members,
            relation,
            member_kind,
        }
    }
}


