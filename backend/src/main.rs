use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use mongodb::{Client, options::ClientOptions, Database, bson::doc};
use bcrypt::{DEFAULT_COST, hash, verify};
mod models;
use crate::models::*;







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
fn main() {
    let password_hash = hash("password", DEFAULT_COST).expect("couldn't hash the password");
    let user = User::new("chuj", password_hash.as_str(),69420, &[]).expect("User with this username already exists");
    dbg!(&user);
    println!("{}", serde_json::to_string(&user).expect("couldn't serialize"));

    let lesson = Lesson::new("Państwo Izrael", "jebać");
    dbg!(&lesson);
    println!("{}", serde_json::to_string(&lesson).expect("couldn't serialize"));

    let answer = Answer {
        lesson : &lesson,
        user : &user,
        answer : "tak".to_owned()
    };

    dbg!(&answer);
    println!("{}", serde_json::to_string(&answer).expect("couldn't serialize"));
}

