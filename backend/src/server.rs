#![deny(clippy::unwrap_used)]

use crate::models::user::*;
use crate::models::task::*;
use axum::http::HeaderValue;
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
use tracing::{info, warn, error};
use tracing::span;
use tower_http::cors::CorsLayer;
use http::Method;

const DEFAULT_IP_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: u32 = 8080;

#[derive(Clone)]
struct AppState {
    db_pool: Arc<Mutex<MySqlPool>>,
}


pub async fn start(db_pool: Arc<Mutex<MySqlPool>>, ip_address: Option<&str>, port: Option<u32>) -> Result<(), std::io::Error> {
    
    let origin = match "http://localhost:3000".parse::<HeaderValue>() {
        Ok(origin) => origin,
        Err(e) => {
            error!("Error parsing origin: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid origin"));
        }
    };
    
    let cors_layer = CorsLayer::very_permissive()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_origin(origin);
    
    let app = Router::new()
        .route("/test", get(test))
        
        .route("/user", put(user::put))
        .route("/user/login", post(user::login))
        .route("/user/register", post(user::register))
        .route("/user/logout", post(user::logout))
        .route("/user/:id", get(user::get).delete(user::delete))
        
        .route("/task/:id", get(task::get))
        .route("/task/random", get(task::get_random))
        .route("/task/next", post(task::get_other_than))
        
        .route("/answer", post(answer::post).put(answer::put).delete(answer::delete))
        .route("/answer/:id", get(answer::get))
        .with_state(AppState { db_pool })
        .layer(cors_layer);
    
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", ip_address.unwrap_or(DEFAULT_IP_ADDRESS), port.unwrap_or(DEFAULT_PORT)))
        .await?;

    info!("Server started on {}:{}", ip_address.unwrap_or(DEFAULT_IP_ADDRESS), port.unwrap_or(DEFAULT_PORT));

    axum::serve(listener, app)
        .await?;  

    Ok(())
}

// Check if the request comes with a valid auth token
async fn validate_token(headers: HeaderMap, tx : &mut Transaction<'static, MySql>) -> Result<Uuid, impl IntoResponse> {
    let header_value = match headers.get(header::AUTHORIZATION) {
        Some(auth) => auth,
        None => {
            warn!("No AUTH header");
            return Err(StatusCode::BAD_REQUEST.into_response());
        }
    };
    
    let token = match header_value.to_str() {
        Ok(token_str) => match Uuid::parse_str(token_str) {
            Ok(id) => id,
            Err(e) => {
                warn!("Couldn't parse AUTH header as UUID\nError: {}", e);
                return Err(StatusCode::BAD_REQUEST.into_response());
            }
        },
        Err(e) => {
            warn!("Couldn't parse AUTH header as string\nError: {}", e);
            return Err(StatusCode::BAD_REQUEST.into_response());
        }
    };

    let auth_result = User::check_token_validity(&Some(token), tx).await;

    match auth_result {
        Ok(result) => {
            if let Err(e) = result {
                match e {
                    AuthorizationError::NoTokenInUser => {
                        warn!("No token in user");
                        return Err(StatusCode::BAD_REQUEST.into_response());
                    }
                    AuthorizationError::TokenExpired => {
                        warn!("Token expired");
                        return Err(StatusCode::FORBIDDEN.into_response());
                    }
                    AuthorizationError::TokenNotInDatabse => {
                        warn!("Token not in database");
                        return Err(StatusCode::UNAUTHORIZED.into_response());
                    }
                    _ => {
                        error!("Other error");
                        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
                    }
                }
            }
        }
        Err(e) => {
            error!("Error while checking token validity: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    };
    
    Ok(token)
}

pub async fn check_authorization(headers: HeaderMap, user_id : &Uuid, tx : &mut Transaction<'static, MySql>) -> Result<(), impl IntoResponse> {
//! Check if the request comes with a valid auth token and the user id is the same as the one in the token
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
                        warn!("Unauthorized attempt!\nToken: {}", token);
                        Err(StatusCode::FORBIDDEN.into_response())
                    },
                    AuthorizationError::TokenExpired => {
                        warn!("Token {} expired.", token);
                        Err(StatusCode::FORBIDDEN.into_response())
                    },
                    AuthorizationError::TokenNotInDatabse => {
                        warn!("Token {} not in database.", token);
                        Err(StatusCode::UNAUTHORIZED.into_response())
                    },
                    _ => {
                        error!("Other authorization error!");
                        Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
                    }
                }
            } else {
                info!("Authorization successful - token: {}", token);
                Ok(())
            }
        },
        Err(e) => {
            error!("Couldn't check authorization!\nError: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

async fn get_transaction(state: AppState) -> Result<Transaction<'static, MySql>, impl IntoResponse> {
    state.db_pool.lock().await
        .begin().await
        .map_err(|e| {error!("Couldn't get transaction!\nError: {}", e); StatusCode::INTERNAL_SERVER_ERROR.into_response()})
}


async fn test() -> impl IntoResponse {
    StatusCode::OK
}

mod user {
    use super::*;
    
    #[derive(serde::Deserialize, Debug)]
    pub struct LoginForm {
        username: String,
        password: String,
    }
    
    pub async fn login(State(state): State<AppState>, Json(form): Json<LoginForm>) -> impl IntoResponse {
        let span = span!(tracing::Level::INFO, "login");
        let _enter = span.enter();
        
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };
        
        match User::login(form.username.clone(), form.password, &mut tx).await {
            Ok(user) => {
                if let Err(e) = tx.commit().await {
                    error!("Couldn't commit transaction!\nError: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
                
                let auth_token = match user.auth_token {
                    Some(token) => token,
                    None => {
                        error!("No auth token in user! THIS SHOULDN'T HAPPEN!");
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                };
                
                axum::http::Response::builder()
                    .status(StatusCode::OK)
                    .header(header::AUTHORIZATION, auth_token.to_string())
                    .body(user.id.to_string().into())
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR.into_response())
            },
            
            Err(e) => match e {
                UserError::BadCredentials => {
                    warn!("Unsuccessful login attempt for user {}, bad credentials", &form.username);
                    StatusCode::FORBIDDEN.into_response()
                },
                UserError::NoSuchUser => {
                    warn!("Unsuccessful login attempt for user {}, no such user", &form.username);
                    StatusCode::NOT_FOUND.into_response()
                }
                UserError::DatabaseError(e) => {
                    error!("Database error: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
                UserError::BadEmail => {
                    error!("Bad email");
                    StatusCode::BAD_REQUEST.into_response()
                }
                UserError::BadPhone => {
                    error!("Bad phone");
                    StatusCode::BAD_REQUEST.into_response()
                }
                UserError::HashError(e) => {
                    error!("Hash error: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
                UserError::MissingFields => {
                    error!("Missing fields");
                    StatusCode::BAD_REQUEST.into_response()
                }
                UserError::UsernameExists => {
                    error!("Username exists, THIS SHOULDN'T HAPPEN, SOMETHING WENT TERRIBLLY WRONG");
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
    }
    
    #[derive(serde::Deserialize, Debug)]
    pub struct RegisterForm {
        username: String,
        password: String,
        email: Option<String>,
        phone: Option<String>,
    }
    
    pub async fn register(
        State(state): State<AppState>,
        Json(form): Json<RegisterForm>,
    ) -> impl IntoResponse {
        let span = span!(tracing::Level::INFO, "register");
        let _enter = span.enter();
        
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };
        
        match User::new(form.username, form.password, form.email, form.phone, &mut tx).await {
            Ok(user) => {
                if let Err(e) = user.create(&mut tx).await {
                    error!("Couldn't create user in database: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
                if let Err(e) = tx.commit().await {
                    error!("Couldn't commit transaction: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
                
                info!("User {} registered successfully", user.username);
                StatusCode::CREATED.into_response()
            }
            Err(e) => match e {
                UserError::UsernameExists => {
                    warn!("Username already exists!");
                    StatusCode::CONFLICT.into_response()
                },
                
                UserError::BadEmail | UserError::BadPhone | UserError::MissingFields => {
                    warn!("Missing fields or bad email or phone.");
                    StatusCode::BAD_REQUEST.into_response()
                }
                
                UserError::DatabaseError(e) => {
                    error!("Database error!\nError: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
                
                UserError::HashError(e) => {
                    error!("Hash error!\nError: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
                
                _ => {
                    error!("Other error!");
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                },
            },
        }
    }
    
    pub async fn put(
        headers: HeaderMap,
        State(state): State<AppState>,
        Json(user_info): Json<UserInfo>,
    ) -> axum::http::Response<axum::body::Body> {
        let span = span!(tracing::Level::INFO, "user update");
        let _enter = span.enter();
        
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };
        
        let user = match user_info.to_user(&mut tx).await {
            Ok(user) => user,
            Err(e) => {
                error!("Couldn't get user from user info: {}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };
        
        if let Err(err_response) = check_authorization(headers, &user.id, &mut tx).await {
            return err_response.into_response();
        }
        
        match user.update(&mut tx).await {
            Ok(_) => {
                if let Err(e) = tx.commit().await {
                    error!("Couldn't commit transaction: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
                info!("Successfully updated user {}", &user.username);
                StatusCode::OK.into_response()
            },
            Err(e) => {
                error!("Couldn't update user {}\nError: {}", user.username, e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            },
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
        
        pub async fn to_user(self, tx: &mut Transaction<'static, MySql>) -> Result<User, sqlx::Error> {
            let id_str = sqlx::query!("SELECT id FROM users WHERE username = ?", self.username)
                .fetch_one(tx.as_mut()).await?.id;
            let id = Uuid::parse_str(&id_str).expect("Invalid UUID in users DB");
            let read_user = User::read(id, tx).await?;
            Ok(User {
                id,
                password_hash: read_user.password_hash,
                username: self.username,
                email: self.email,
                phone: self.phone,
                bio: self.bio,
                friends: self.friends,
                level: self.level,
                progress: self.progress,
                auth_token: read_user.auth_token,
            })
        }
    }
    
    pub async fn get(
        headers: HeaderMap,
        State(state): State<AppState>,
        Path(id_str): Path<String>,
    ) -> impl IntoResponse {
        let span = span!(tracing::Level::INFO, "user get");
        let _enter = span.enter();
        
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
                warn!("Invalid UUID: {}", id_str);
                return StatusCode::BAD_REQUEST.into_response();
            }
        };
    
        match User::read(id, &mut tx).await {
            Ok(user) => {
                if let Err(e) = tx.commit().await {
                    error!("Couldn't commit transaction: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
                
                let json = match serde_json::to_string(&UserInfo::from_user(user)) {
                    Ok(json) => json,
                    Err(e) => {
                        error!("Couldn't serialize user info to JSON: {}", e);
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                };
                
                match axum::http::Response::builder()
                    .status(StatusCode::OK)
                    .body(json.into()) {
                        Ok(response) => response,
                        Err(e) => {
                            error!("Couldn't build response: {}", e);
                            StatusCode::INTERNAL_SERVER_ERROR.into_response()
                        }
                    }
            }
            Err(e) => {
                error!("Couldn't read user from db: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            },
        }
    }
    
    pub async fn delete(
        headers: HeaderMap,
        State(state): State<AppState>,
        Path(id_str) : Path<String>,
    ) -> impl IntoResponse {
        let span = span!(tracing::Level::INFO, "user delete");
        let _enter = span.enter();
        
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };
        
        let id = match Uuid::parse_str(&id_str) {
            Ok(id) => id,
            Err(e) => {
                warn!("Invalid UUID: {}", e);
                return StatusCode::BAD_REQUEST.into_response();
            }
        };
    
        if let Err(err_response) = check_authorization(headers, &id, &mut tx).await {
            return err_response.into_response();
        }
        
        match User::delete(id, &mut tx).await {
            Ok(_) => {
                if let Err(e) = tx.commit().await {
                    error!("Couldn't commit transaction: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
                info!("Successfully deleted user {}", id);
                StatusCode::NO_CONTENT.into_response()
            },
            Err(e) => {
                error!("Couldn't delete user: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()},
        }
    }
    
    pub async fn logout(
        headers: HeaderMap,
        State(state): State<AppState>,
    ) -> impl IntoResponse {
        let span = span!(tracing::Level::INFO, "user logout");
        let _enter = span.enter();
        
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };
        
        let token = match validate_token(headers, &mut tx).await {
            Ok(token) => token,
            Err(err) => return err.into_response()
        };
        
        match User::logout(token, &mut tx).await {
            Ok(_) => {
                if let Err(e) = tx.commit().await {
                    error!("Couldn't commit transaction: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
                StatusCode::OK.into_response()
            },
            Err(e) => {
                error!("Couldn't log out! \nError: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()},
        }
    }
}

mod task {
    use reqwest::RequestBuilder;

    use super::*;
   
    pub async fn get(
        Path(id_str): Path<String>, 
        State(state): State<AppState>) -> impl IntoResponse {
        
        let span = span!(tracing::Level::INFO, "task get");
        let _enter = span.enter();
        
        let id = match Uuid::parse_str(&id_str) {
            Ok(id) => id,
            Err(e) => {
                warn!("Invalid UUID: {}", e);
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
                if let Err(e) = tx.commit().await {
                    error!("Couldn't commit transaction: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
                info!("Successfully read task {}", id);
                Json(task).into_response()
            },
            
            Err(e) => match e {
                sqlx::Error::RowNotFound => {
                    warn!("Entry {} not found", id);
                    StatusCode::NOT_FOUND.into_response()
                },
                
                _ => {
                    error!("Other error!");
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                },
            },
        }
    }
    
    pub async fn get_random(State(state): State<AppState>) -> impl IntoResponse {
        use sqlx::query;
        use rand::seq::SliceRandom;
        use axum::http::Response;
        
        let span = span!(tracing::Level::INFO, "task get random");
        let _enter = span.enter();
        
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };
        
        let task_id = match query!("SELECT id FROM tasks")
            .fetch_all(tx.as_mut()).await {
                Ok(records) => {
                    let mut rng = rand::thread_rng();
                    match records.choose(&mut rng) {
                        Some(record) => {
                            match uuid::Uuid::parse_str(&record.id) {
                                Ok(id) => id,
                                Err(e) => {
                                    error!("Couldn't parse UUID: {}", e);
                                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                                }
                            }
                        }
                        None => {
                            warn!("No tasks found");
                            return StatusCode::NOT_FOUND.into_response();
                        }
                    }
                },
                Err(e) => {
                    error!("Couldn't get random task: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            };
        
        let task = match Task::read(task_id, &mut tx).await {
            Ok(task) => task,
            Err(e) => {
                error!("Couldn't read task: {}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };
        
        let json = serde_json::to_string(&task);
        
        if let Err(e) = json {
            error!("Couldn't serialize task: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        
        match Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(json.expect("This result is an Err, THIS SHOULDN'T HAPPEN").into()) {
                Ok(response) => response,
                Err(e) => {
                    error!("Couldn't build response: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
    }
    
    pub async fn get_other_than(
        State(state): State<AppState>,
        Json(task_ids): Json<Vec<Uuid>>,
    ) -> impl IntoResponse {
        use sqlx::query;
        use axum::http::Response;
        use rand::seq::SliceRandom;
        
        let span = span!(tracing::Level::INFO, "task get other than");
        let _enter = span.enter();
        
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };
        
        let task_id = match query!("SELECT id FROM tasks")
            .fetch_all(tx.as_mut()).await {
                Ok(records) => {
                    let mut rng = rand::thread_rng();
                    loop {
                        match records.choose(&mut rng) {
                            Some(record) => {
                                match uuid::Uuid::parse_str(&record.id) {
                                    Ok(id) => {
                                        if !task_ids.contains(&id) {
                                            break id;
                                        }
                                    },
                                    Err(e) => {
                                        error!("Couldn't parse UUID: {}", e);
                                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                                    }
                                }
                            }
                            None => {
                                warn!("No tasks found");
                                return StatusCode::NOT_FOUND.into_response();
                            }
                        }
                    }
                },
                Err(e) => {
                    error!("Couldn't get random task: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            };
        
        let task = match Task::read(task_id, &mut tx).await {
            Ok(task) => task,
            Err(e) => {
                error!("Couldn't read task: {}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };
        
        let json = serde_json::to_string(&task);
        
        if let Err(e) = json {
            error!("Couldn't serialize task: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        
        match Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(json.expect("This result is an Err, THIS SHOULDN'T HAPPEN").into()) {
                Ok(response) => response,
                Err(e) => {
                    error!("Couldn't build response: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
    }
}

mod answer {
    use axum::http::Response;
    use super::*;
    use serde::Deserialize;
    use crate::models::answer::*;
    
    pub async fn get(
        Path(id_str) : Path<String>, 
        State(state): State<AppState>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let span = span!(tracing::Level::INFO, "answer get");
        let _enter = span.enter();
        
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };
        
        if let Err(err_response) = validate_token(headers, &mut tx).await {
            return err_response.into_response();
        }
        
        let id = match Uuid::parse_str(&id_str) {
            Ok(id) => id,
            Err(e) => {
                warn!("Invalid UUID: {}", e);
                return StatusCode::BAD_REQUEST.into_response();
            }
        };
        
        match Answer::read(id, &mut tx).await {
            Ok(answer) => {
                if let Err(e) = tx.commit().await {
                    error!("Couldn't commit transaction: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
                
                match answer {
                    Some(answer) => {
                        info!("Successfully read answer {}", id);
                        
                        let serialized = match answer.serialize() {
                            Ok(serialized) => serialized,
                            Err(e) => {
                                error!("Couldn't serialize answer: {}", e);
                                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                            }
                        };
                        
                        Response::builder()
                            .status(StatusCode::OK)
                            .body(serialized.into())
                            .unwrap_or_else(|_| {
                                error!("Couldn't build response");
                                StatusCode::INTERNAL_SERVER_ERROR.into_response()
                            })
                    },
                    None => {
                        warn!("Answer {} not found", id);
                        StatusCode::NOT_FOUND.into_response()
                    },
                }
            },
            Err(e) => match e {
                sqlx::Error::RowNotFound => {
                    warn!("Answer {} not found", id);
                    StatusCode::NOT_FOUND.into_response()
                },
                _ => {
                    error!("Other error!");
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                },
            },
        }
    }
    
    #[derive(Deserialize, Debug)]
    pub struct AnswerForm {
        pub user_id: Uuid,
        pub task_id: Uuid,
        pub content: Option<crate::models::answer::AnswerContent> 
    }
    
    impl AnswerForm {
        fn into_answer(self) -> Answer {
            Answer {
                id: Uuid::new_v4(),
                user_id: self.user_id,
                task_id: self.task_id,
                content: self.content,
            }
        }
    }
    
    pub async fn post(
        headers: HeaderMap,
        State(state): State<AppState>,
        Json(answer_form): Json<AnswerForm>,
    ) -> impl IntoResponse {
        use serde_json::json;
        
        let span = span!(tracing::Level::INFO, "answer post");
        let _enter = span.enter();
        
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };
        
        if let Err(err_response) = check_authorization(headers, &answer_form.user_id, &mut tx).await {
            return err_response.into_response();
        }
       
        let answer = answer_form.into_answer();
        
        match answer.create(&mut tx).await {
            Ok(id) => {
                info!("Answer successfully created.");
                
                let verify_result = match answer.verify(&mut tx).await {
                    Ok(verify_result) => verify_result,
                    Err(e) => {
                        error!("Couldn't verify answer: {:#?}", e);
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                };
                
                let mut json = match serde_json::to_value(&verify_result) {
                    Ok(json) => json,
                    Err(e) => {
                        error!("Couldn't serialize verify result: {}", e);
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                };
                
                json["id"] = json!(id);
                
                if let Err(e) = tx.commit().await {
                    error!("Couldn't commit transaction: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
                
                let response = Response::builder()
                    .status(StatusCode::CREATED)
                    .header("Location", format!("/answer/{}", id))
                    .body(json.to_string().into());
                match response {
                    Ok(response) => response,
                    Err(e) => {
                        error!("Couldn't create response: {}", e);
                        StatusCode::INTERNAL_SERVER_ERROR.into_response()
                    }
                }
            },
            Err(e) => {
                error!("Couldn't create answer: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            },
        } 
    }
    
    pub async fn put(
        headers: HeaderMap,
        State(state): State<AppState>,
        Json(answer): Json<Answer>,
    ) -> impl IntoResponse {
        let span = span!(tracing::Level::INFO, "answer put");
        let _enter = span.enter();
        
        let mut tx = match get_transaction(state).await {
            Ok(tx) => tx,
            Err(e) => return e.into_response(),
        };
        
        if let Err(err_response) = check_authorization(headers, &answer.user_id, &mut tx).await {
            return err_response.into_response();
        }
        
        match answer.update(&mut tx).await {
            Ok(_) => {
                info!("Answer {} updated", answer.id);
                StatusCode::OK.into_response()
            },
            Err(e) => {
                error!("Couldn't update answer {}: {}", answer.id, e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            },
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
            Err(e) => {
                warn!("Invalid UUID: {}", e);
                return StatusCode::BAD_REQUEST.into_response();
            }
        };
        
        if let Err(err_response) = check_authorization(headers, &id, &mut tx).await {
            return err_response.into_response();
        }
        
        match Answer::delete(id, &mut tx).await {
            Ok(_) => {
                info!("Answer {} successfully deleted", id);
                StatusCode::NO_CONTENT.into_response()
            },
            Err(e) => {
                error!("Couldn't delete answer {}: {}", id, e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
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
