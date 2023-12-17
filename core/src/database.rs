// use sqlx::migrate;
// use sqlx::postgres::{PgPool, PgPoolOptions};
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;
// use tracing::info;

/// Creates a connection pool for the database defined at DATABASE_URL.
pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL needs to be defined.");

    let db = Database::connect(&url).await?;

    // let pool = PgPoolOptions::new().connect(&url).await?;

    // info!("Applying migrations.");
    // migrate!("./migrations").run(&pool).await?;

    Ok(db)
}
