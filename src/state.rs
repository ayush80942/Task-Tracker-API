use std::sync::{Arc, Mutex};
use crate::models::Task;

pub type SharedState = Arc<AppState>;

#[derive(Default)]
pub struct AppState {
    pub tasks: Mutex<Vec<Task>>,
}
