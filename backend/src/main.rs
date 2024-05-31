use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use mongodb::{Client, options::ClientOptions, Database, bson::doc};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct Lesson<'a> {
    id: Uuid,
    title: &'a str, // We don't want the title to be mutable/growable, so we can store it in what is essentially a [u8]
    content: &'a str,
}

impl <'a> Lesson<'a> {
    fn new(title : &'a str, content : &'a str) -> Lesson<'a> {
        Lesson {
            id : uuid::Uuid::new_v4(),
            title,
            content
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct User<'a> {
    #[serde(with = "uuid::serde::simple")]
    id: Uuid,
    username: &'a str,
    progress: i32,
}

impl <'a> User<'a> {
    fn new(username : &'a str, progress : i32, current_users : &[User]) -> Option<User<'a>> {
        if current_users.iter()
        .map(|user| user.username)
        .any(|existing_username| existing_username == username) {
            return None;
        }
        Some(User {
            id : Uuid::new_v4(),
            username,
            progress
        })
    }
}

#[derive(Debug)]
struct AnswerData<'a> {
    lesson : &'a Lesson<'a>,
    user : &'a User<'a>,
    answer: &'a str
}

impl <'a> Serialize for AnswerData<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("AnswerData", 3)?;
        state.serialize_field("lesson", self.lesson.id.to_string().as_str())?;
        state.serialize_field("user", self.user.id.to_string().as_str())?;
        state.serialize_field("answer", self.answer)?;
        state.end()
            
    }
}

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
    let user = User::new("chuj", 69420, &[]).expect("User with this username already exists");
    dbg!(&user);
    println!("{}", serde_json::to_string(&user).expect("couldn't serialize"));

    let lesson = Lesson::new("Państwo Izrael", "jebać");
    dbg!(&lesson);
    println!("{}", serde_json::to_string(&lesson).expect("couldn't serialize"));

    let answer = AnswerData {
        lesson : &lesson,
        user : &user,
        answer : "tak"
    };

    dbg!(&answer);
    println!("{}", serde_json::to_string(&answer).expect("couldn't serialize"));
}