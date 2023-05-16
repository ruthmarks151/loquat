use std::future::Future;

use gloo_net::http;

const INDEX_REQ_URL: &str = "/api/fan_sizes";

pub fn get(id: String) -> impl Future<Output = Result<gloo_net::http::Response, gloo_net::Error>> {
    let req_url = format!("{}/{}", INDEX_REQ_URL, id);
    http::Request::get(req_url.as_str()).send()
}
