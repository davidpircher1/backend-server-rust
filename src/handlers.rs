use crate::model::{self, TaskResponse, TaskRequest, TaskError};
use axum::{
    Json, extract::{Path, State}, http::StatusCode 
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


pub async fn read_tasks(pool: &SqlitePool) -> Result<Vec<TaskResponse>, model::TaskError> {
    let tasks = sqlx::query_as::<_, TaskResponse>("SELECT id, title, priority FROM tasks")
        .fetch_all(pool) // Chceme VŠETKY riadky
        .await
        .map_err(|_| TaskError::DatabaseError)?;

    Ok(tasks)
}


pub async fn read_tasks_handler( State(pool): State<SqlitePool>) -> (StatusCode, Json<Vec<TaskResponse>>) {
    match read_tasks(&pool).await {
        Ok(tasks) => {
            (StatusCode::OK, Json(tasks))
        },
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new()))
        }
    }
}


pub async fn read_task(pool: &SqlitePool, id: &String) -> Result<Option<TaskResponse>, model::TaskError> {
    let task = sqlx::query_as::<_, TaskResponse>(
            "SELECT id, title, priority FROM tasks WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool) // Vráti Ok(Some(task)) alebo Ok(None)
        .await
        .map_err(|_| TaskError::DatabaseError)?;

    Ok(task)
}


pub async fn read_task_handler( State(pool): State<SqlitePool>, Path(id): Path<String>) -> (StatusCode, String) {
    match read_task(&pool, &id).await {
        Ok(task) => {
            if task.is_some() {
                (StatusCode::OK, serde_json::to_string(&task).unwrap())
            } else {
                (StatusCode::NOT_FOUND, "There is no record with this id.".to_string())
            }
        },
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Database invalid".to_string())
        }
    }
}