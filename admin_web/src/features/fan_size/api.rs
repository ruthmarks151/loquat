use gloo_net::http::Request;
use loquat_common::api::GetFanSizeResponse;

const INDEX_REQ_URL: &str = "/api/fan_sizes";

pub async fn get_fan_size(fan_size_id: String) -> Result<GetFanSizeResponse, String> {
    let req_url = format!("{}/{}", INDEX_REQ_URL, fan_size_id);
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
