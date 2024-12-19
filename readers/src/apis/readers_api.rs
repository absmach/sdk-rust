/*
 * SuperMQ reader service
 *
 * HTTP API for reading messages. Some useful links: - [The SuperMQ repository](https://github.com/absmach/supermq) 
 *
 * The version of the OpenAPI document: 0.15.1
 * Contact: info@abstractmachines.fr
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`get_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMessagesError {
    Status400(),
    Status401(),
    Status500(),
    UnknownValue(serde_json::Value),
}


/// Retrieves a list of messages sent to specific channel. Due to performance concerns, data is retrieved in subsets. The API readers must ensure that the entire dataset is consumed either by making subsequent requests, or by increasing the subset size of the initial request. 
pub async fn get_messages(configuration: &configuration::Configuration, chan_id: &str, limit: Option<i32>, offset: Option<i32>, publisher: Option<&str>, name: Option<&str>, v: Option<&str>, vb: Option<bool>, vs: Option<&str>, vd: Option<&str>, from: Option<f64>, to: Option<f64>, aggregation: Option<&str>, interval: Option<&str>) -> Result<models::MessagesPage, Error<GetMessagesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/channels/{chanId}/messages", local_var_configuration.base_path, chanId=crate::apis::urlencode(chan_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = publisher {
        local_var_req_builder = local_var_req_builder.query(&[("Publisher", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder = local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = v {
        local_var_req_builder = local_var_req_builder.query(&[("v", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = vb {
        local_var_req_builder = local_var_req_builder.query(&[("vb", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = vs {
        local_var_req_builder = local_var_req_builder.query(&[("vs", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = vd {
        local_var_req_builder = local_var_req_builder.query(&[("vd", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = aggregation {
        local_var_req_builder = local_var_req_builder.query(&[("aggregation", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = interval {
        local_var_req_builder = local_var_req_builder.query(&[("interval", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetMessagesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

