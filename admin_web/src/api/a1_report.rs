use std::future::Future;

use gloo_net::http;
use loquat_common::models::A1Standard2010Report;
use serde::Serialize;
use serde_json::value::Serializer;

const INDEX_REQ_URL: &str = "/api/a1_2010_report";

pub fn get(id: String) -> impl Future<Output = Result<gloo_net::http::Response, gloo_net::Error>> {
    let req_url = format!("{}/{}", INDEX_REQ_URL, id);
    http::Request::get(req_url.as_str()).send()
}

pub fn put(
    payload: A1Standard2010Report<()>,
) -> impl Future<Output = Result<gloo_net::http::Response, gloo_net::Error>> {
    let req_url = format!("{}/{}", INDEX_REQ_URL, payload.id);
    http::Request::put(req_url.as_str())
        .body(payload.serialize(Serializer).unwrap().to_string())
        .send()
}

pub fn post(
    payload: A1Standard2010Report<()>,
) -> impl Future<Output = Result<gloo_net::http::Response, gloo_net::Error>> {
    http::Request::post(INDEX_REQ_URL)
        .body(payload.serialize(Serializer).unwrap().to_string())
        .send()
}
