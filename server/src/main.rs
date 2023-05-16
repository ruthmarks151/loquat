use std::path::PathBuf;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Extension, Router,
};
use loquat_server::controllers;
use sqlx::PgPool;
use tower_http::services::{ServeDir, ServeFile};

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migrations failed :(");

    for statement in [
        "INSERT INTO fan_serieses (fan_series_id, fan_type) VALUES ('SKYPLUME G2-ELLV DMF', 'centrifugal') ON CONFLICT DO NOTHING;",
        "INSERT INTO fan_serieses (fan_series_id, fan_type) VALUES ('SKYPLUME G1-ELLV DMF', 'mixed_flow') ON CONFLICT DO NOTHING;",
        "INSERT INTO fan_sizes (fan_size_id, fan_series_id, diameter, outlet_area) VALUES ('SKYPLUME G1-ELLV DMF-150', 'SKYPLUME G1-ELLV DMF', 18.25, 200.5) ON CONFLICT DO NOTHING;",
        "INSERT INTO fan_sizes (fan_size_id, fan_series_id, diameter, outlet_area) VALUES ('SKYPLUME G1-ELLV DMF-250', 'SKYPLUME G1-ELLV DMF', 25.0, 300.2) ON CONFLICT DO NOTHING;",
        "INSERT INTO public.a1_2010_reports (a1_2010_report_id,fan_size_id,rpm,determinations) VALUES
        ('3214','SKYPLUME G1-ELLV DMF-150',1750.0,'[{\"cfm\": 1243.0, \"static_pressure\": 2.0, \"brake_horsepower\": 0.7}]') ON CONFLICT DO NOTHING;",
    ] {
        sqlx::query(statement).execute(&pool).await.expect("Data insert failed");
    }

    // In production, serve from the root static folder
    // In dev, this is empty and the frontend proxies to anything /api to this server
    let serve_dir = get_service(
        ServeDir::new(static_folder.clone())
            .fallback(ServeFile::new(static_folder.join("index.html"))),
    )
    .handle_error(handle_error);

    let router = Router::new()
        .route("/api/fan_series", get(controllers::fan_series::index))
        .route("/api/fan_series/:id", get(controllers::fan_series::get))
        .route("/api/fan_sizes", get(controllers::fan_size::index))
        .route("/api/fan_sizes/:id", get(controllers::fan_size::get))
        .route(
            "/api/a1_2010_report/:id",
            get(controllers::a1_2010_report::get),
        )
        .fallback_service(serve_dir)
        .layer(Extension(pool));
    Ok(router.into())
}
