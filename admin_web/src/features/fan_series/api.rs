use gloo_net::http::Request;
use loquat_common::api::fan_series::{GetResponse, IndexResponse};

const INDEX_REQ_URL: &str = "/api/fan_series";

pub async fn index_fan_serieses() -> Result<IndexResponse, String> {
    let resp = Request::get(INDEX_REQ_URL).send().await.unwrap();
    if !resp.ok() {
        Err(format!(
            "Error fetching data {} ({})",
            resp.status(),
            resp.status_text()
        ))
    } else {
        resp.json().await.map_err(|err| err.to_string())
    }
}

pub async fn get_fan_series(fan_series_id: String) -> Result<GetResponse, String> {
    let req_url = format!("{}/{}", INDEX_REQ_URL, fan_series_id);
    let resp = Request::get(req_url.as_str()).send().await.unwrap();
    if !resp.ok() {
        Err(format!(
            "Error fetching data {} ({})",
            resp.status(),
            resp.status_text()
        ))
    } else {
        resp.json().await.map_err(|err| err.to_string())
    }
}
