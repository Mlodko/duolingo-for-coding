use actix_web::{web, App, HttpServer, HttpResponse};
use sqlx::{mysql::{MySqlPool, MySqlPoolOptions}, Row}; // Importujemy Row
use rand::seq::SliceRandom; // Importujemy SliceRandom z rand
use bcrypt::verify; // Importujemy verify z bcrypt
use dotenv::dotenv; // Importujemy dotenv do ładowania .env
use std::env;

#[derive(Debug, sqlx::FromRow, Clone)]
struct Task {
    id: i32,
    content: Option<String>, // content może być None, więc używamy Option<String>
    isOpen: bool,
    answer: Option<String>, // Podobnie, answer jest opcjonalne
    answerA: Option<String>,
    answerB: Option<String>,
    answerC: Option<String>,
    answerD: Option<String>,
    correctAnswerClosed: Option<String>, // Oczekujemy char dla correctAnswerClosed
    correctAnswerOpen: Option<String>,
    valueOfTask: i32,
}



async fn get_random_tasks(pool: &MySqlPool) -> Result<Vec<Task>, sqlx::Error> {
    let tasks: Vec<Task> = sqlx::query_as!(
        Task,
        "SELECT id, content, isOpen as `isOpen: bool`, answer, answerA, answerB, answerC, answerD, correctAnswerClosed, correctAnswerOpen, valueOfTask FROM tasks WHERE valueOfTask <= 10"
    )
  
    .fetch_all(pool)
    .await?;

    let mut rng = rand::thread_rng();
    let mut selected_tasks: Vec<Task> = Vec::new();
    let mut total_value = 0;

    for task in tasks.choose_multiple(&mut rng, 5) {
        selected_tasks.push(task.clone().to_owned()); // Clonujemy, aby mieć własną kopię task
        total_value += task.valueOfTask;
        if total_value >= 10 {
            break;
        }
    }

    Ok(selected_tasks)
}

async fn verify_user(pool: &MySqlPool, name: &str, password: &str) -> Result<bool, sqlx::Error> {
    let row =  sqlx::query!(
        
        "SELECT password FROM users WHERE name = ?",
        name
    )
    .fetch_one(pool)
    .await?;
   Ok(row.password==password)
    
}



#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok(); // Ładujemy zmienne środowiskowe z .env

    // Pobieramy URL bazy danych z .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Tworzymy pool połączeń
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone()) // Używamy app_data zamiast data, aby udostępnić pool
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}