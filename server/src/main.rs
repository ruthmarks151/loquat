use std::path::PathBuf;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Json, Router,
};
use tower_http::services::{ServeDir, ServeFile};

use loquat_common::Fan;

async fn get_fan(Path(id): Path<String>) -> Json<Fan> {
    let fan = Fan {
        id: id,
        name: "Spinmax 9000".to_string(),
    };
    Json(fan)
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    let serve_dir = get_service(
        ServeDir::new(static_folder.clone())
            .fallback(ServeFile::new(static_folder.join("index.html"))),
    )
    .handle_error(handle_error);

    let router = Router::new()
        .route("/api/fans/:id", get(get_fan))
        .fallback_service(serve_dir);
    Ok(router.into())
}
