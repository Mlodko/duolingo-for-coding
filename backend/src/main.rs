use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use mongodb::{Client, options::ClientOptions, Database, bson::doc};
use tokio;

#[derive(Serialize, Deserialize)]
struct Lesson {
    id: String,
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    progress: i32,
}

#[derive(Serialize, Deserialize)]
struct AnswerData {
    lesson_id: String,
    user_id: String,
    answer: String,
}

async fn get_lesson(db: web::Data<Database>, id: web::Path<String>) -> impl Responder {
    let collection = db.collection::<Lesson>("lessons");
    let filter = doc! { "id": id.into_inner() };
    match collection.find_one(filter, None).await {
        Ok(Some(lesson)) => HttpResponse::Ok().json(lesson),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn submit_answer(data: web::Json<AnswerData>) -> impl Responder {
    // Implementacja wysyłania odpowiedzi do API OpenAI i zwracania wyniku
    HttpResponse::Ok().finish()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("programming_lessons");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/lesson/{id}", web::get().to(get_lesson))
            .route("/submit_answer", web::post().to(submit_answer))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}