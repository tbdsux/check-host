use std::{collections::HashMap, thread, time::Duration};

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

fn sleep(time: u8) {
    thread::sleep(Duration::from_secs(time.into()));
}

pub fn check_http(
    host: &str,
    nodes: u8,
    wait: u8,
) -> Result<HashMap<String, Option<Vec<Vec<Value>>>>, reqwest::Error> {
    let url = utils::check_url_builder(host, nodes, "http");
    let init_check = utils::api_request::<CheckRequestResponse>(url.as_str())?;

    sleep(wait);

    let result_url = utils::result_url_builder(&init_check.request_id);
    let check_result =
        utils::api_request::<HashMap<String, Option<Vec<Vec<Value>>>>>(result_url.as_str())?;

    Ok(check_result)
}

pub fn check_ping(
    host: &str,
    nodes: u8,
    wait: u8,
) -> Result<HashMap<String, Option<Vec<Vec<Vec<Value>>>>>, reqwest::Error> {
    let url = utils::check_url_builder(host, nodes, "ping");
    let init_check = utils::api_request::<CheckRequestResponse>(url.as_str())?;

    sleep(wait);

    let result_url = utils::result_url_builder(&init_check.request_id);
    let check_result =
        utils::api_request::<HashMap<String, Option<Vec<Vec<Vec<Value>>>>>>(result_url.as_str())?;

    Ok(check_result)
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TcpCheckResponse {
    Ok { address: String, time: f64 },
    Err { error: String },
}

pub fn check_tcp(
    host: &str,
    nodes: u8,
    wait: u8,
) -> Result<HashMap<String, Option<Vec<TcpCheckResponse>>>, reqwest::Error> {
    let url = utils::check_url_builder(host, nodes, "tcp");
    let init_check = utils::api_request::<CheckRequestResponse>(url.as_str())?;

    sleep(wait);

    let result_url = utils::result_url_builder(&init_check.request_id);
    let check_result = utils::api_request(result_url.as_str())?;

    Ok(check_result)
}
