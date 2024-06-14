use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use mongodb::{Client, options::ClientOptions, Database, bson::doc};
use bcrypt::{DEFAULT_COST, hash, verify};
mod models;
mod server;
use sqlx::mysql::MySqlPoolOptions;
use std::env;






/*
    fn from_str(json_text : &str, existing_users : &'a [User], existing_lessons : &'a [Lesson]) -> Result<Answer<'a>, serde_json::Error> {
        let answer_data : AnswerData = serde_json::from_str(json_text)?;
        let lessons : Vec<&Lesson> = existing_lessons.iter()
            .filter(|lesson| lesson.id == answer_data.lesson_id)
            .collect();
        let matching_lesson = lessons.first();

        if matching_lesson.is_none() {
            return Err(serde_json::Error::custom("Can't find lesson in provided existing lessons"));
        }

        let users : Vec<&User> = existing_users.iter()
            .filter(|user| user.id == answer_data.user_id)
            .collect();
        let matching_user = users.first();

        if matching_user.is_none() {
            return Err(serde_json::Error::custom("Can't find user in provided existing users"));
        }
    

        Ok(Answer {
            lesson : matching_lesson.unwrap(), // there is no way of there being None so it's okay I think 
            user :  matching_user.unwrap(),
            answer : answer_data.answer.to_owned()
        })
    }
*/



/*
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
async fn mainn() -> std::io::Result<()> {
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
*/
async fn main() -> Result<(), sqlx::Error> {
    // Pobierz URL bazy danych z zmiennej środowiskowej
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Utwórz pulę połączeń
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Przykładowe zapytanie
    let row: (i64,) = sqlx::query_as("SELECT ?")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    println!("Fetched row: {:?}", row);

    Ok(())
}

