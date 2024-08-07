use crate::{
    database,
    models::{task::*, user::*},
};
use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

const IP_ADDRESS: &str = "127.0.0.1";
const PORT: u32 = 8080;

#[derive(Clone)]
struct AppState {
    db_pool: Arc<Mutex<MySqlPool>>,
}



pub async fn start(db_pool: Arc<Mutex<MySqlPool>>) -> Result<(), std::io::Error> {
    let app = Router::new()
        .route("/test", get(test))
        .route(
            "/user",
            get(user::login).post(user::register).put(user::update),
        )
        .route("/task/:id", get(task::get))
        .route("/answer", get(answer::get_answer).post(answer::post_answer))
        .with_state(AppState { db_pool });
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", IP_ADDRESS, PORT))
        .await?;
    axum::serve(listener, app)
        .await?;
    Ok(())
}

async fn authorize(headers: HeaderMap, state: &AppState) -> Result<(), impl IntoResponse> {
    let token = match headers.get(header::AUTHORIZATION).unwrap().to_str() {
        Ok(token_str) => match Uuid::parse_str(token_str) {
            Ok(id) => id,
            Err(_) => {
                return Err(StatusCode::BAD_REQUEST.into_response());
            }
        },
        Err(_) => {
            return Err(StatusCode::BAD_REQUEST.into_response());
        }
    };

    let db_pool = state.db_pool.lock().await;

    let auth_result = User::check_authorization(&Some(token), &db_pool).await;

    match auth_result {
        Ok(result) => {
            if let Err(e) = result {
                match e {
                    AuthorizationError::NoTokenInUser => {
                        return Err((StatusCode::BAD_REQUEST, "No token in user").into_response());
                    }
                    AuthorizationError::TokenExpired => {
                        return Err((StatusCode::FORBIDDEN, "Token expired").into_response());
                    }
                    AuthorizationError::TokenNotInDatabse => {
                        return Err((StatusCode::UNAUTHORIZED, "Invalid token").into_response());
                    }
                }
            }
        }
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    };

    Ok(())
}

async fn test() -> impl IntoResponse {
    StatusCode::OK
}

mod user {
    use super::*;
    
    #[derive(serde::Deserialize, Debug)]
    pub struct LoginForm {
        username: String,
        password_hash: String,
    }
    
    pub async fn login(State(state): State<AppState>, Json(form): Json<LoginForm>) -> impl IntoResponse {
        let db_pool = state.db_pool.lock().await;
        
        match User::login(form.username, form.password_hash, &db_pool).await {
            Ok(user) => {
                axum::http::Response::builder()
                    .status(StatusCode::OK)
                    .header(header::AUTHORIZATION, user.auth_token.unwrap().to_string())
                    .body("".into())
                    .unwrap()
            },
            
            Err(e) => match e {
                UserError::BadCredentials => StatusCode::FORBIDDEN.into_response(),
                _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            },
        }
    }
    
    #[derive(serde::Deserialize, Debug)]
    pub struct RegisterForm {
        username: String,
        password_hash: String,
        email: Option<String>,
        phone: Option<String>,
    }
    
    pub async fn register(
        State(state): State<AppState>,
        Json(form): Json<RegisterForm>,
    ) -> impl IntoResponse {
        let db_pool = state.db_pool.lock().await;
    
        match User::new(
            form.username,
            form.password_hash,
            form.email,
            form.phone,
            &db_pool).await {
                
            Ok(user) => {
                if (user.insert(&db_pool).await).is_err() {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
                StatusCode::CREATED.into_response()
            }
            Err(e) => match e {
                UserError::UsernameExists => StatusCode::CONFLICT.into_response(),
                UserError::BadEmail | UserError::BadPhone | UserError::MissingFields => {
                    StatusCode::BAD_REQUEST.into_response()
                }
                _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            },
        }
    }
    
    pub async fn update(
        headers: HeaderMap,
        State(state): State<AppState>,
        Json(user): Json<User>,
    ) -> axum::http::Response<axum::body::Body> {
        if let Err(err_response) = authorize(headers, &state).await {
            return err_response.into_response();
        }
    
        let db_pool = state.db_pool.lock().await;
    
        match user.update(&db_pool).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

mod task {
    use super::*;
   
    pub async fn get(Path(id_str): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
        let id = Uuid::parse_str(&id_str);
        println!("Got Task get by id request: {:?}", &id);
    
        if id.is_err() {
            return StatusCode::BAD_REQUEST.into_response();
        }
    
        let id = id.unwrap();
        let db_pool = state.db_pool.lock().await;
        let task = Task::read(id, &db_pool).await;
        match task {
            Ok(task) => Json(task).into_response(),
    
            Err(e) => match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND.into_response(),
    
                _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            },
        }
    }
}

mod answer {
    use super::*;
    
    pub async fn get_answer() -> impl IntoResponse {
        todo!("Getting answers not yet implemented");
    }
    
    pub async fn post_answer() -> impl IntoResponse {
        todo!("Posting answers not yet implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;
    use tokio::runtime::Runtime;

    #[tokio::test]
    async fn test_endpoint() {
        let rt = Runtime::new().unwrap();
        let db_pool = database::get_database_connection_pool().await.unwrap();
        rt.block_on(async {
            // Start the server in a separate Tokio task
            tokio::spawn(start(db_pool));

            // Give the server a moment to start
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

            let response = reqwest::get(format!("http://{}:{}/test", IP_ADDRESS, PORT))
                .await
                .expect("Failed to execute request.");

            assert_eq!(response.status(), StatusCode::OK);
        });
    }
}
