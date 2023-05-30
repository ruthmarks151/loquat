use std::future::Future;

use gloo_net::http;
use loquat_common::api;
use serde::Serialize;
use serde_json::value::Serializer;

const INDEX_REQ_URL: &str = "/api/fan_series";

pub fn index() -> impl Future<Output = Result<gloo_net::http::Response, gloo_net::Error>> {
    http::Request::get(INDEX_REQ_URL).send()
}

pub fn get(id: String) -> impl Future<Output = Result<gloo_net::http::Response, gloo_net::Error>> {
    let req_url = format!("{}/{}", INDEX_REQ_URL, id);
    http::Request::get(req_url.as_str()).send()
}

pub fn put(
    payload: api::fan_series::UpdateBody,
) -> impl Future<Output = Result<gloo_net::http::Response, gloo_net::Error>> {
    let req_url = format!("{}/{}", INDEX_REQ_URL, payload.id);
    http::Request::put(req_url.as_str())
        .header("Content-Type", "application/json")
        .body(payload.serialize(Serializer).unwrap().to_string())
        .send()
}

pub fn post(
    payload: api::fan_series::UpdateBody,
) -> impl Future<Output = Result<gloo_net::http::Response, gloo_net::Error>> {
    http::Request::post(INDEX_REQ_URL)
        .header("Content-Type", "application/json")
        .body(payload.serialize(Serializer).unwrap().to_string())
        .send()
}