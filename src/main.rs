#[path = "actorRouter.rs"]
mod actor_router;
mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from the .env file
    dotenvy::dotenv().ok();

    // Get the database URL from the environment variable
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection pool to the PostgreSQL database
    let pool = db::create_pool(&database_url).await?;

    let app = actor_router::router(pool);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Server is running on http://127.0.0.1:3000");
    println!("Fetch all actors at http://127.0.0.1:3000/actors");
    axum::serve(listener, app).await?;

    Ok(())
}
