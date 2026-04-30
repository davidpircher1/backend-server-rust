use crate::model::*;
use axum::{
    Json, extract::{Path, State}, http::StatusCode, response::IntoResponse
};
use sqlx::{sqlite::SqlitePool};
use crate::tasks::*;
use crate::users::*;


pub async fn create_task_handler( State(pool): State<SqlitePool>, Json(task): Json<TaskRequest>) -> impl IntoResponse {
    match create_task(task, &pool).await {
        Ok(task) => {
            (StatusCode::CREATED, serde_json::to_string(&task).unwrap()).into_response()
        },
        Err(e) => {
            match e {
                TaskError::AppErrors(msg) => {
                    (StatusCode::BAD_REQUEST, msg).into_response()
                }, 
                TaskError::DatabaseError => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response()
                },
                _ => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Error").into_response()
                }
            }
        }
    }
}


pub async fn read_tasks_handler( State(pool): State<SqlitePool>) -> impl IntoResponse {
    match read_tasks(&pool).await {
        Ok(tasks) => {
            (StatusCode::OK, Json(tasks)).into_response()
        },
        Err(TaskError::DatabaseError) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response()
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Invalid ID").into_response()
        }
    }
}


pub async fn read_task_handler( State(pool): State<SqlitePool>, Path(id): Path<String>) -> impl IntoResponse {
    match read_task(&pool, &id).await {
        Ok(task) => {
            if task.is_some() {
                (StatusCode::OK, serde_json::to_string(&task).unwrap()).into_response()
            } else {
                (StatusCode::NOT_FOUND, "There is no record with this id.").into_response()
            }
        },
        Err(TaskError::DatabaseError) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Database invalid").into_response()
        },
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Unkown error").into_response()
        }
    }
}

pub async fn update_task_handler(State(pool): State<SqlitePool>, Path(id): Path<String>, Json(task): Json<TaskRequest>) -> impl IntoResponse {
    match update_task(&pool, &id, task).await {
        Ok(t) => {
            if t {
                (StatusCode::OK, "Update success").into_response()
            } else {
                (StatusCode::BAD_REQUEST, "Update unsuccessful").into_response()
            }
        }
        Err(e) => {
            match e {
                TaskError::AppErrors(msg) => {
                    (StatusCode::BAD_REQUEST, msg).into_response()
                }, 
                TaskError::DatabaseError => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response()
                },
                _ => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Error").into_response()
                }
            }
        }
    }
}

pub async fn delete_task_handler(State(pool): State<SqlitePool>, Path(id): Path<String>) -> impl IntoResponse {
    match delete_task(&pool, &id).await {
        Ok(t) => {
            if t {
                (StatusCode::OK, "Delete success").into_response()
            } else {
                (StatusCode::BAD_REQUEST, "Delete unsuccessful").into_response()
            }
        }
        Err(TaskError::DatabaseError) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Database invalid").into_response()
        },
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Unkown error").into_response()
        }
    }
}

pub async fn register_user_handler(State(pool): State<SqlitePool>, Json(user): Json<User>) -> impl IntoResponse {
    match register_user(user, &pool).await {
        Ok(_) => {
            (StatusCode::CREATED, "Registered successfuly").into_response()
        },
        Err(e) => {
            match e {
                TaskError::AppErrors(msg) => {
                    (StatusCode::BAD_REQUEST, msg).into_response()
                },
                TaskError::DatabaseError => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Chyba databázy").into_response()
                },
                _ => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Neznama chyba").into_response()
                }
            }
        }
    }

}