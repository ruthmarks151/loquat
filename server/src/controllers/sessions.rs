use axum::{
    body::BoxBody,
    extract::{Extension, Json},
    http::Request,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use jwt::{Header, Token};
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Audience	Must be your Firebase project ID, the unique identifier for your Firebase project, which can be found in the URL of that project's console.
    exp: usize, // Expiration time	Must be in the future. The time is measured in seconds since the UNIX epoch.
    iat: usize, // Issued-at time	Must be in the past. The time is measured in seconds since the UNIX epoch.
    iss: String, // Issuer	Must be "https://securetoken.google.com/<projectId>", where <projectId> is the same project ID used for aud above.
    // nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Subject	Must be a non-empty string and must be the uid of the user or device.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostSessionResponse {
    success: bool,
    message: String,
    redirect: Option<String>,
}

const AUTHORIZED_UIDS: [&str; 2] = [
    "eY9lIYhF4QTZHdz8VAHoyOeCm1S2", // RRM
    "DYTCeA3RJNZpks2fQr2clWLKFE83", // JKJ
];

fn get_authed_user_id(cookie_jar: &CookieJar) -> Option<String> {
    let authorization = cookie_jar.get("Authorization")?;
    let authorization_token = authorization.value().strip_prefix("Bearer ")?;

    // Algorithm::RS256
    let unverified: Token<Header, Claims, _> = Token::parse_unverified(authorization_token)
        .map_err(|err| err.to_string())
        .ok()?;

    let claims = unverified.claims();
    Some(claims.sub.clone())
}

pub async fn post(
    Extension(_pool): Extension<PgPool>,
    cookie_jar: CookieJar, // Json(PostBody): Json<()>,
) -> Result<Json<PostSessionResponse>, String> {
    let claimed_id = get_authed_user_id(&cookie_jar);

    if AUTHORIZED_UIDS
        .iter()
        .any(|authed_id| Some(authed_id.to_string()) == claimed_id)
    {
        Ok(Json(PostSessionResponse {
            success: true,
            message: "Authenticated and Authorized! Redirecting...".to_string(),
            redirect: Some("/fan_series".to_string()),
        }))
    } else {
        Ok(Json(PostSessionResponse {
            success: false,
            message: format!(
                "Authenticated successfully.<br>However, this account is not presently authorized.<br>Please ask for user ID '{}' to be authorized",
                claimed_id.unwrap_or("None".to_string())
            ),
            redirect: None,
        }))
    }
}

pub async fn auth_middleware<B>(request: Request<B>, next: Next<B>) -> Response {
    // do something with `request`...
    let cookie_jar = CookieJar::from_headers(request.headers());
    let claimed_id = get_authed_user_id(&cookie_jar);

    println!("Route: {}", request.uri());

    if request.uri().query() == Some("/static/login.html")
        || request.uri().to_string().starts_with("/api/sessions")
    {
        next.run(request).await
    } else if AUTHORIZED_UIDS
        .iter()
        .any(|authed_id| Some(authed_id.to_string()) == claimed_id)
    {
        next.run(request).await
    } else {
        Response::builder()
            .status(401)
            .body(BoxBody::default())
            .unwrap()
    }
}
