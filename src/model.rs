use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TaskRequest {
    pub title: String,
    pub priority: i32,
}

#[derive(Deserialize, Serialize)]
pub struct TaskResponse {
    pub id: String,
    pub title: String,
    pub priority: i32,
}

pub enum TaskError {
    InvalidTitle,
    InvalidPriority,
    DatabaseError,
}