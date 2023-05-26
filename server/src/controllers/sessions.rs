use axum::extract::{Extension, Json};
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

const AUTHORIZED_UIDS: [&str; 1] = ["eY9lIYhF4QTZHdz8VAHoyOeCm1S2"];

pub async fn post(
    Extension(_pool): Extension<PgPool>,
    cookie_jar: CookieJar, // Json(PostBody): Json<()>,
) -> Result<Json<PostSessionResponse>, String> {
    let authorization = cookie_jar.get("Authorization").unwrap();
    let authorization_token = authorization.value().strip_prefix("Bearer ").unwrap();

    // Algorithm::RS256
    let unverified: Token<Header, Claims, _> =
        Token::parse_unverified(authorization_token).map_err(|err| err.to_string())?;

    let claims = unverified.claims();
    let claimed_id = claims.sub.clone();

    if AUTHORIZED_UIDS
        .iter()
        .find(|authed_id| **authed_id == claimed_id.as_str())
        .is_some()
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
                "Authenticated successfully. However, this account is not presently authorized. Please ask for user ID '{}' to be authorized",
                claimed_id
            )
            .to_string(),
            redirect: None,
        }))
    }
}
