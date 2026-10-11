// Import the necessary modules from the sqlx crate
use sqlx::postgres::PgPoolOptions;

// Create a function to create a connection pool to the PostgreSQL database
pub async fn create_pool(database_url: &str) -> Result<sqlx::PgPool, sqlx::Error> {
    // Create a connection pool to the PostgreSQL database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    println!("Connected to the database successfully.");

    Ok(pool)
}

