/*
 * WarframeStat.us API
 *
 * Simple API for data from the game Warframe. [Parser Docs](https://wfcd.github.io/warframe-worldstate-parser/) [Items Types](https://github.com/WFCD/warframe-items/blob/master/index.d.ts) 
 *
 * The version of the OpenAPI document: living
 * Contact: tobiah@protonmail.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`platform_rivens_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlatformRivensGetError {
    Status400(crate::models::InlineResponse400),
    Status500(crate::models::InlineResponse400),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`platform_rivens_search_query_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlatformRivensSearchQueryGetError {
    Status400(crate::models::InlineResponse400),
    Status500(crate::models::InlineResponse400),
    UnknownValue(serde_json::Value),
}


/// Data about averages, deviations, medians, miniums, and maxes for all rivens for the provided platform
pub async fn platform_rivens_get(configuration: &configuration::Configuration, platform: crate::models::Platform, accept_language: Option<crate::models::Language>, language: Option<crate::models::Language>) -> Result<::std::collections::HashMap<String, crate::models::Riven>, Error<PlatformRivensGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/{platform}/rivens", local_var_configuration.base_path, platform=platform);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = language {
        local_var_req_builder = local_var_req_builder.query(&[("language", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = accept_language {
        local_var_req_builder = local_var_req_builder.header("Accept-Language", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PlatformRivensGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Data about averages, deviations, medians, miniums, and maxes for rivens whose name match the query for the provided platform
pub async fn platform_rivens_search_query_get(configuration: &configuration::Configuration, platform: crate::models::Platform, query: &str) -> Result<::std::collections::HashMap<String, crate::models::Riven>, Error<PlatformRivensSearchQueryGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/{platform}/rivens/search/{query}", local_var_configuration.base_path, platform=platform, query=crate::apis::urlencode(query));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PlatformRivensSearchQueryGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

