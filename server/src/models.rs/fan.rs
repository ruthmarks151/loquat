use sqlx::PgPool;

#[derive(FromRow)]
pub use loquat_common::models::fan::Fan;