use crate::models::user::*;
use crate::models::task::*;
use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    routing::post,
    routing::put,
    Json, Router,
};
use sqlx::MySql;
use sqlx::MySqlPool;
use sqlx::Transaction;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

const DEFAULT_IP_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: u32 = 8080;

#[derive(Clone)]
struct AppState {
    db_pool: Arc<Mutex<MySqlPool>>,
}

pub async fn start(db_pool: Arc<Mutex<MySqlPool>>, ip_address: Option<&str>, port: Option<u32>) -> Result<(), std::io::Error> {
    let app = Router::new()
        .route("/test", get(test))
        
        .route("/user", put(user::put))
        .route("/user/login", get(user::login))
        .route("/user/register", post(user::register))
        .route("/user/:id", get(user::get).delete(user::delete))
        
        .route("/task/:id", get(task::get))
        
        .route("/answer", post(answer::post).put(answer::put).delete(answer::delete))
        .route("/answer/:id", get(answer::get))
        .with_state(AppState { db_pool });
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", ip_address.unwrap_or(DEFAULT_IP_ADDRESS), port.unwrap_or(DEFAULT_PORT)))
        .await?;
    axum::serve(listener, app)
        .await?;
    Ok(())
}

// Check if the request comes with a valid auth token
async fn validate_token(headers: HeaderMap, tx : &mut Transaction<'static, MySql>) -> Result<Uuid, impl IntoResponse> {
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

    let auth_result = User::check_token_validity(&Some(token), tx).await;

    match auth_result {
        Ok(result) => {
            if let Err(e) = result {
                match e {
                    AuthorizationError::NoTokenInUser => {
                        return Err(StatusCode::BAD_REQUEST.into_response());
                    }
                    AuthorizationError::TokenExpired => {
                        return Err(StatusCode::FORBIDDEN.into_response());
                    }
                    AuthorizationError::TokenNotInDatabse => {
                        return Err(StatusCode::UNAUTHORIZED.into_response());
                    }
                    _ => {
                        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
                    }
                }
            }
        }
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    };
    
    Ok(token)
}

// Check if the request comes with a valid auth token and the user id is the same as the one in the token
pub async fn check_authorization(headers: HeaderMap, user_id : &Uuid, tx : &mut Transaction<'static, MySql>) -> Result<(), impl IntoResponse> {
    let token = match validate_token(headers.clone(), tx).await {
        Ok(token) => token,
        Err(e) => {
            return Err(e.into_response());
        }
    };
    
    match User::check_authorization(token, user_id, tx).await {
        Ok(result) => {
            if let Err(e) = result {
                match e {
                    AuthorizationError::NotAuthorized => {
                        Err(StatusCode::FORBIDDEN.into_response())
                    }
                    _ => {
                        Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
                    }
                }
            } else {
                Ok(())
            }
        },
        Err(_) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

async fn get_transaction(state: AppState) -> Result<Transaction<'static, MySql>, impl IntoResponse> {
    state.db_pool.lock().await
        .begin().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
}


async fn test() -> impl IntoResponse {
    StatusCode::OK
}

mod user {
    use axum::http::Response;

    use super::*;
    
    #[derive(serde::Deserialize, Debug)]
    pub struct LoginForm {
        username: String,
        password_hash: String,
    }
    
    pub async fn login(State(state): State<AppState>, Json(form): Json<LoginForm>) -> impl IntoResponse {
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };
        
        match User::login(form.username, form.password_hash, &mut tx).await {
            Ok(user) => {
                tx.commit().await.unwrap();
                
                axum::http::Response::builder()
                    .status(StatusCode::OK)
                    .header(header::AUTHORIZATION, user.auth_token.unwrap().to_string())
                    .body(user.id.to_string().into())
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
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };
        
        match User::new(form.username, form.password_hash, form.email, form.phone, &mut tx).await {
            Ok(user) => {
                if (user.create(&mut tx).await).is_err() {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
                tx.commit().await.unwrap();
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
    
    pub async fn put(
        headers: HeaderMap,
        State(state): State<AppState>,
        Json(user): Json<User>,
    ) -> axum::http::Response<axum::body::Body> {
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };
        
        if let Err(err_response) = check_authorization(headers, &user.id, &mut tx).await {
            return err_response.into_response();
        }
        
        match user.update(&mut tx).await {
            Ok(_) => {
                tx.commit().await.unwrap();
                StatusCode::OK.into_response()
            },
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
    
    #[derive(serde::Serialize ,serde::Deserialize, Debug)]
    pub struct UserInfo {
        pub username: String,
        pub email: Option<String>,
        pub phone: Option<String>,
        pub bio: Option<String>,
        pub friends: Vec<Uuid>,
        pub level: UserLevel,
        pub progress: UserProgress
    }
    
    impl UserInfo {
        pub fn from_user(user: User) -> Self {
            Self {
                username: user.username,
                email: user.email,
                phone: user.phone,
                bio: user.bio,
                friends: user.friends,
                level: user.level,
                progress: user.progress,
            }
        }
    }
    
    pub async fn get(
        headers: HeaderMap,
        State(state): State<AppState>,
        Path(id_str): Path<String>,
    ) -> impl IntoResponse {
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };
        
        if let Err(err_response) = validate_token(headers, &mut tx).await {
            return err_response.into_response();
        }
    
        let id = match Uuid::parse_str(&id_str) {
            Ok(id) => id,
            Err(_) => {
                return StatusCode::BAD_REQUEST.into_response();
            }
        };
    
        match User::read(id, &mut tx).await {
            Ok(user) => {
                tx.commit().await.unwrap();
                axum::http::Response::builder()
                    .status(StatusCode::OK)
                    .body(serde_json::to_string(&UserInfo::from_user(user)).unwrap().into())
                    .unwrap()
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
    
    pub async fn delete(
        headers: HeaderMap,
        State(state): State<AppState>,
        Path(id_str) : Path<String>,
    ) -> impl IntoResponse {
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };
        
        let id = match Uuid::parse_str(&id_str) {
            Ok(id) => id,
            Err(_) => {
                return StatusCode::BAD_REQUEST.into_response();
            }
        };
    
        if let Err(err_response) = check_authorization(headers, &id, &mut tx).await {
            return err_response.into_response();
        }
        
        match User::delete(id, &mut tx).await {
            Ok(_) => {
                tx.commit().await.unwrap();
                StatusCode::OK.into_response()
            },
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

mod task {
    use super::*;
   
    pub async fn get(Path(id_str): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
        let id = match Uuid::parse_str(&id_str) {
            Ok(id) => id,
            Err(_) => {
                return StatusCode::BAD_REQUEST.into_response();
            }
        };
    
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };

        let task = Task::read(id, &mut tx).await;
        match task {
            Ok(task) => {
                tx.commit().await.unwrap();
                Json(task).into_response()
            },
            
            Err(e) => match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND.into_response(),
                
                _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            },
        }
    }
}

mod answer {
    use axum::http::Response;
    use super::*;
    use crate::models::answer::*;
    
    pub async fn get(
        Path(id_str) : Path<String>, 
        State(state): State<AppState>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };
        
        if let Err(err_response) = validate_token(headers, &mut tx).await {
            return err_response.into_response();
        }
        
        let id = match Uuid::parse_str(&id_str) {
            Ok(id) => id,
            Err(_) => {
                return StatusCode::BAD_REQUEST.into_response();
            }
        };
        
        match Answer::read(id, &mut tx).await {
            Ok(answer) => {
                tx.commit().await.unwrap();
                match answer {
                    Some(answer) => {
                        Response::builder()
                            .status(StatusCode::OK)
                            .body(answer.serialize().unwrap().into())
                            .unwrap()
                    },
                    None => StatusCode::NOT_FOUND.into_response(),
                }
            },
            Err(e) => match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND.into_response(),
                _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            },
        }
    }
    
    pub async fn post(
        headers: HeaderMap,
        State(state): State<AppState>,
        Json(answer): Json<Answer>,
    ) -> impl IntoResponse {
        
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };
        
        if let Err(err_response) = check_authorization(headers, &answer.user_id, &mut tx).await {
            return err_response.into_response();
        }
       
        match answer.create(&mut tx).await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        } 
    }
    
    pub async fn put(
        headers: HeaderMap,
        State(state): State<AppState>,
        Json(answer): Json<Answer>,
    ) -> impl IntoResponse {
        
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };
        
        if let Err(err_response) = check_authorization(headers, &answer.user_id, &mut tx).await {
            return err_response.into_response();
        }
        
        match answer.update(&mut tx).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
    
    pub async fn delete(
        headers: HeaderMap,
        State(state): State<AppState>,
        Path(id_str): Path<String>,
    ) -> impl IntoResponse {
        
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };
        
        let id = match Uuid::parse_str(&id_str) {
            Ok(id) => id,
            Err(_) => {
                return StatusCode::BAD_REQUEST.into_response();
            }
        };
        
        if let Err(err_response) = check_authorization(headers, &id, &mut tx).await {
            return err_response.into_response();
        }
        
        match Answer::delete(id, &mut tx).await {
            Ok(_) => StatusCode::NO_CONTENT.into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database;
    use reqwest::StatusCode;

    #[tokio::test]
    async fn test_endpoint() {
        let db_pool = database::get_database_connection_pool(None).await.unwrap();

        // Start the server in a separate Tokio task
        tokio::spawn(start(db_pool, None, None));

        // Give the server a moment to start
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        let response = reqwest::get(format!("http://{}:{}/test", DEFAULT_IP_ADDRESS, DEFAULT_PORT))
            .await
            .expect("Failed to execute request.");

        assert_eq!(response.status(), StatusCode::OK);
    }
}
