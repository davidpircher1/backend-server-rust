use crate::model::{self, TaskResponse, TaskRequest, TaskError};
use axum::http::StatusCode;
use uuid::Uuid;
use axum::Json;

pub fn create_task(task: TaskRequest) -> Result<TaskResponse, model::TaskError> {
    if task.title.is_empty() {
        return Err(TaskError::InvalidTitle);
    } else if task.title.len() > 16 || task.title.len() < 3 {
        return Err(TaskError::InvalidTitle);
    } else if task.priority == 0 {
        return Err(TaskError::InvalidPriority);
    }

    Ok(TaskResponse { id: Uuid::new_v4().to_string(), title: task.title, priority: task.priority })
}


pub async fn create_task_handler(Json(task): Json<TaskRequest>) -> (StatusCode, String) {
    match create_task(task) {
        Ok(task) => {
            (StatusCode::CREATED, serde_json::to_string(&task).unwrap())
        },
        Err(TaskError::InvalidTitle) => {
            (StatusCode::BAD_REQUEST, "InvalidTitle".to_string())
        },
        Err(TaskError::InvalidPriority) => {
            (StatusCode::BAD_REQUEST, "InvalidPriority".to_string())
        }
    }
}