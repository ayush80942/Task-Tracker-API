use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::models::{Task, TaskInput};
use crate::state::SharedState;

pub async fn health_check() -> &'static str {
    "✅ API is running!"
}

pub async fn get_tasks(State(state): State<SharedState>) -> Json<Vec<Task>> {
    let tasks = state.tasks.lock().unwrap();
    Json(tasks.clone())
}

pub async fn get_task(Path(id): Path<Uuid>, State(state): State<SharedState>) -> Result<Json<Task>, StatusCode> {
    let tasks = state.tasks.lock().unwrap();
    tasks.iter().find(|t| t.id == id).cloned().map(Json).ok_or(StatusCode::NOT_FOUND)
}

pub async fn create_task(State(state): State<SharedState>, Json(input): Json<TaskInput>) -> Json<Task> {
    let new_task = Task {
        id: Uuid::new_v4(),
        title: input.title,
        completed: false,
    };

    let mut tasks = state.tasks.lock().unwrap();
    tasks.push(new_task.clone());

    Json(new_task)
}

pub async fn update_task(
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
    Json(update): Json<Task>,
) -> Result<Json<Task>, StatusCode> {
    let mut tasks = state.tasks.lock().unwrap();
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.title = update.title;
        task.completed = update.completed;
        return Ok(Json(task.clone()));
    }
    Err(StatusCode::NOT_FOUND)
}

pub async fn delete_task(Path(id): Path<Uuid>, State(state): State<SharedState>) -> StatusCode {
    let mut tasks = state.tasks.lock().unwrap();
    let before = tasks.len();
    tasks.retain(|task| task.id != id);
    if tasks.len() < before {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn toggle_task(Path(id): Path<Uuid>, State(state): State<SharedState>) -> Result<Json<Task>, StatusCode> {
    let mut tasks = state.tasks.lock().unwrap();
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = !task.completed;
        return Ok(Json(task.clone()));
    }
    Err(StatusCode::NOT_FOUND)
}
