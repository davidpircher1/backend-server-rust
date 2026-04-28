use axum::{
    routing::post,
    Router,
};

mod handlers;
mod model;


#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", post(handlers::create_task_handler));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}