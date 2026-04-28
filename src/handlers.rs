use crate::model::{self, TaskResponse, TaskRequest, TaskError};
use axum::{
    extract::State, // State je v extract
    Json, 
    http::StatusCode, 
};
use sqlx::{sqlite::SqlitePool};

pub async fn create_task(task: TaskRequest, pool: &SqlitePool) -> Result<TaskResponse, model::TaskError> {
    if task.title.is_empty() {
        return Err(TaskError::InvalidTitle);
    } else if task.title.len() > 16 || task.title.len() < 3 {
        return Err(TaskError::InvalidTitle);
    } else if task.priority == 0 {
        return Err(TaskError::InvalidPriority);
    }

    let id = uuid::Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO tasks (id, title, priority) VALUES (?, ?, ?)")
        .bind(&id)
        .bind(&task.title)
        .bind(&task.priority)
        .execute(pool)
        .await
        .map_err(|_| TaskError::DatabaseError)?;

    Ok(TaskResponse { id: id, title: task.title, priority: task.priority })
}


pub async fn create_task_handler( State(pool): State<SqlitePool>, Json(task): Json<TaskRequest>) -> (StatusCode, String) {
    match create_task(task, &pool).await {
        Ok(task) => {
            (StatusCode::CREATED, serde_json::to_string(&task).unwrap())
        },
        Err(TaskError::InvalidTitle) => {
            (StatusCode::BAD_REQUEST, "InvalidTitle".to_string())
        },
        Err(TaskError::InvalidPriority) => {
            (StatusCode::BAD_REQUEST, "InvalidPriority".to_string())
        },
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "DatabaseError".to_string())
        }
    }
}