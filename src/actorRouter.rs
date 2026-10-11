use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
struct Actor {
    actor_id: i32,
    first_name: String,
    last_name: String,
    birth_date: Option<String>,
    created_at: Option<String>,
}

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/actors", get(get_actors))
        .with_state(pool)
}

async fn get_actors(State(pool): State<PgPool>) -> Result<Json<Vec<Actor>>, StatusCode> {
    let rows = sqlx::query_as::<_, (i32, String, String, Option<String>, Option<String>)>(
        "SELECT actor_id, first_name, last_name, birth_date::text, created_at::text \
		 FROM actor ORDER BY actor_id",
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let actors = rows
        .into_iter()
        .map(
            |(actor_id, first_name, last_name, birth_date, created_at)| Actor {
                actor_id,
                first_name,
                last_name,
                birth_date,
                created_at,
            },
        )
        .collect();

    Ok(Json(actors))
}
