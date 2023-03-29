use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use serde::de::DeserializeOwned;
use url::Url;

static API_URL: &str = "https://check-host.net";

pub fn api_request<T>(uri: &str) -> Result<T, reqwest::Error>
where
    T: DeserializeOwned,
{
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let resp = client.get(uri).headers(headers).send()?;

    let data = resp.json::<T>()?;
    Ok(data)
}

pub fn check_url_builder(host: &str, nodes: u8, check_type: &str) -> Url {
    let mut url = Url::parse(API_URL).unwrap();
    url.set_path(&format!("check-{}", check_type));
    url.query_pairs_mut().append_pair("host", &host[..]);

    if nodes > 0 {
        url.query_pairs_mut()
            .append_pair("max_nodes", &nodes.to_string()[..]);
    }

    url
}

pub fn result_url_builder(id: &str) -> Url {
    let mut result_url = Url::parse(API_URL).unwrap();
    result_url.set_path(&format!("check-result/{}", id));

    result_url
}
