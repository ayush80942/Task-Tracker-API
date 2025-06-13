mod handlers;
mod models;
mod state;

use axum::{routing::{get, post, put, delete, patch}, Router};
use handlers::*;
use state::AppState;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::default());

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/tasks", get(get_tasks).post(create_task))
        .route("/tasks/:id", get(get_task).put(update_task).delete(delete_task))
        .route("/tasks/:id/toggle", patch(toggle_task))
        .with_state(state);

    println!("🚀 Server running at http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
