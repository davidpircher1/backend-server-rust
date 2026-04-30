use validator::Validate;
use sqlx::sqlite::{SqlitePool};

use crate::model::{User, TaskError};

pub async fn register_user(user: User, pool: &SqlitePool) -> Result<(), TaskError>{
    user.validate().map_err(|e| {TaskError::AppErrors(e.to_string())})?;
    
    sqlx::query("INSERT INTO users (name, mail, password_hash) VALUES(?,?,?)")
    .bind(&user.name)
    .bind(&user.mail)
    .bind(&user.passwd)
    .execute(pool)
    .await
    .map_err(|_| { TaskError::DatabaseError })?;
    Ok(())
}