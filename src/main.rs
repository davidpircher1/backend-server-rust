use axum::{
    routing::post,
    routing::get,
    Router,
};

mod handlers;
mod model;

use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() {
    // 1. pripojenie do db
    let pool = SqlitePool::connect("sqlite:tasks.db").await.unwrap();

    // 2. query
    let query = "
        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY, 
            title TEXT NOT NULL,
            priority INTEGER
        );
    ";

    // 3. spustenie query
    sqlx::query(query)
        .execute(&pool)
        .await    
        .unwrap();

    // build our application with a single route
    let app = Router::new()
        .route("/tasks", post(handlers::create_task_handler))
        .route("/tasks", get(handlers::read_tasks_handler))
        .route("/tasks/:id", get(handlers::read_task_handler)).with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}