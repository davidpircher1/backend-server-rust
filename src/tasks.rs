use crate::model::*;
use sqlx::sqlite::{SqlitePool};
use validator::Validate;

pub async fn create_task(task: TaskRequest, pool: &SqlitePool) -> Result<TaskResponse, TaskError> {

    task.validate().map_err(|e|{TaskError::AppErrors(e.to_string())})?;

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

pub async fn read_tasks(pool: &SqlitePool) -> Result<Vec<TaskResponse>, TaskError> {
    let tasks = sqlx::query_as::<_, TaskResponse>("SELECT id, title, priority FROM tasks")
        .fetch_all(pool) // Chceme VŠETKY riadky
        .await
        .map_err(|_| TaskError::DatabaseError)?;

    Ok(tasks)
}

pub async fn read_task(pool: &SqlitePool, id: &String) -> Result<Option<TaskResponse>, TaskError> {
    let task = sqlx::query_as::<_, TaskResponse>(
            "SELECT id, title, priority FROM tasks WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool) // Vráti Ok(Some(task)) alebo Ok(None)
        .await
        .map_err(|_| TaskError::DatabaseError)?;

    Ok(task)
}

pub async fn update_task(pool: &SqlitePool, id: &String, task: TaskRequest) -> Result<bool, TaskError> {
    task.validate().map_err(|e|{TaskError::AppErrors(e.to_string())})?;


    let result = sqlx::query("UPDATE tasks SET title=?, priority=? WHERE id=?")
    .bind(&task.title)
    .bind(&task.priority)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|_| TaskError::DatabaseError)?;

    Ok(result.rows_affected() > 0)
}

pub async fn delete_task(pool: &SqlitePool, id: &String) -> Result<bool, TaskError> {
    let result = sqlx::query("DELETE FROM tasks WHERE id=?")
    .bind(id)
    .execute(pool)
    .await
    .map_err(|_| TaskError::DatabaseError)?;

    Ok(result.rows_affected() > 0)
}