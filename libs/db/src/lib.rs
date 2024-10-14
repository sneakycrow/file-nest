use sqlx::postgres::PgPool;

/// Function to establish a connection to the PostgreSQL database
pub async fn connect_to_database() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    tracing::debug!("Creating DB connection Pool");
    let pool = PgPool::connect(&database_url).await?;

    Ok(pool)
}
