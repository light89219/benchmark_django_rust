mod db;
mod handlers;
mod models;

use axum::{Router, routing::get};
use sqlx::SqlitePool;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

#[tokio::main]
async fn main() {
    let pool = db::init_db().await;
    let state = AppState { db: pool };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/todos", get(handlers::list_todos).post(handlers::create_todo))
        .route(
            "/api/todos/{id}",
            get(handlers::get_todo)
                .put(handlers::update_todo)
                .patch(handlers::patch_todo)
                .delete(handlers::delete_todo),
        )
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8001").await.unwrap();
    println!("Server running on http://127.0.0.1:8001");
    axum::serve(listener, app).await.unwrap();
}
