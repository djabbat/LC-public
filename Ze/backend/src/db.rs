use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing::info;

pub async fn init(database_url: &str) -> Result<PgPool, sqlx::Error> {
    info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Run migrations
    info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}