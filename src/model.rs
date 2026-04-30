use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Deserialize, Serialize, Validate)]
pub struct TaskRequest {
    #[validate(length(min = 3, message = "Title must be atleast 3 characters long"))]
    pub title: String,
    #[validate(range(min = 1,max = 5, message = "Priority must be number 1-5."))]
    pub priority: i32,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct TaskResponse {
    pub id: String,
    pub title: String,
    pub priority: i32,
}

#[derive(Debug, Validate, Deserialize)]
pub struct User {
    #[validate(length(min = 3, message = "Name must be atleast 3 characters long."))]
    pub name: String, 
    #[validate(email(message = "Wrong email."))]
    pub mail: String,
    #[validate(length(min = 8, message = "Password must be atleast 3 characters long"))]
    pub passwd: String,
}

pub enum TaskError {
    DatabaseError,
    AppErrors(String),
}
