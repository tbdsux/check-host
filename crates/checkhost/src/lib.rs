use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

mod utils;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CheckRequestResponse {
    nodes: HashMap<String, Vec<String>>,
    ok: u8,
    permanent_link: String,
    request_id: String,
}

pub fn check_http(
    host: &str,
    nodes: u8,
) -> Result<HashMap<String, Option<Vec<Vec<Value>>>>, reqwest::Error> {
    let url = utils::check_url_builder(host, nodes, "http");
    let init_check = utils::api_request::<CheckRequestResponse>(url.as_str())?;

    let result_url = utils::result_url_builder(&init_check.request_id);
    let check_result =
        utils::api_request::<HashMap<String, Option<Vec<Vec<Value>>>>>(result_url.as_str())?;

    Ok(check_result)
}

pub fn check_ping(
    host: &str,
    nodes: u8,
) -> Result<HashMap<String, Option<Vec<Vec<Vec<Value>>>>>, reqwest::Error> {
    let url = utils::check_url_builder(host, nodes, "ping");
    let init_check = utils::api_request::<CheckRequestResponse>(url.as_str())?;

    let result_url = utils::result_url_builder(&init_check.request_id);
    let check_result =
        utils::api_request::<HashMap<String, Option<Vec<Vec<Vec<Value>>>>>>(result_url.as_str())?;

    Ok(check_result)
}
