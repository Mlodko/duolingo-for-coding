use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use sqlx::mysql::MySqlPoolOptions;
use std::env;

const IP_ADDRESS : &str = "127.0.0.1";
const PORT : u32 = 8080;

pub async fn start() {
    let app = Router::new()
        .route("/test", get(test))
        .route("/user", get(login_user).post(register_user))
        .route("/lesson", get(get_lesson))
        .route("/answer", get(get_answer).post(post_answer));
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

async fn get_lesson() -> impl IntoResponse {
    todo!("Getting lessons not yet implemented");
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

    #[test]
    fn test_endpoint() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            // Start the server in a separate Tokio task
            tokio::spawn(start());

            // Give the server a moment to start
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

            let response = reqwest::get(format!("http://{}:{}/test", IP_ADDRESS, PORT)).await.expect("Failed to execute request.");

            assert_eq!(response.status(), StatusCode::OK);
        });
    }
}