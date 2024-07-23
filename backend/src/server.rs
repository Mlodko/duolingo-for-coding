
use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::get, Router, Json};
use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::{Acquire, MySqlPool, query};
use crate::{database, models::Task};
use uuid::Uuid;


const IP_ADDRESS : &str = "127.0.0.1";
const PORT : u32 = 8080;

#[derive(Clone)]
struct AppState {
    db_pool: Arc<Mutex<MySqlPool>>
}

pub async fn start(db_pool: Arc<Mutex<MySqlPool>>) {
    let db_pool = database::initialize_db_pool().await;

    let app = Router::new()
        .route("/test", get(test))
        .route("/user", get(login_user).post(register_user))
        .route("/task/:id", get(get_task))
        .route("/answer", get(get_answer).post(post_answer))
        .with_state(AppState { db_pool });
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", IP_ADDRESS, PORT)).await.expect("Couldn't start tokio listener");
    axum::serve(listener, app).await.expect("Couldn't serve axum app");
}

async fn test() -> impl IntoResponse {
    StatusCode::OK
}

async fn login_user() -> impl IntoResponse {
    todo!("User login not yet implemented");
}

async fn register_user() -> impl IntoResponse {
    todo!("User registration not yet implemented");
}

async fn get_task(
    Path(id_str): Path<String>,
    State(state): State<AppState>
) -> impl IntoResponse {
    let id = Uuid::parse_str(&id_str);
    println!("Got Task get by id request: {:?}", &id);
    
    if id.is_err() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    
    let id = id.unwrap();
    let db_pool = state.db_pool.lock().await;
    let task = Task::read(id, &db_pool).await;
    match task {
        Ok(task) => {
            Json(task).into_response()
        }
        
        Err(e) => {
            match e {
                sqlx::Error::RowNotFound => {
                    StatusCode::NOT_FOUND.into_response()
                }
                
                _ => {
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
    }
}

async fn get_answer() -> impl IntoResponse {
    todo!("Getting answers not yet implemented");
}

async fn post_answer() -> impl IntoResponse {
    todo!("Posting answers not yet implemented");
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;
    use tokio::runtime::Runtime;

    #[tokio::test]
    async fn test_endpoint() {
        let rt = Runtime::new().unwrap();
        let db_pool = database::initialize_db_pool().await;
        rt.block_on(async {
            // Start the server in a separate Tokio task
            tokio::spawn(start(db_pool));

            // Give the server a moment to start
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

            let response = reqwest::get(format!("http://{}:{}/test", IP_ADDRESS, PORT)).await.expect("Failed to execute request.");

            assert_eq!(response.status(), StatusCode::OK);
        });
    }
}