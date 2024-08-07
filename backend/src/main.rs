//use actix_web::{web, App, HttpServer, Responder, HttpResponse};
//use mongodb::{Client, options::ClientOptions, Database, bson::doc};
//use bcrypt::{DEFAULT_COST, hash, verify};
mod models;
mod server;
mod database;



#[tokio::main]
async fn main() {
    let db_pool = match database::get_database_connection_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Error connecting to database: {}", e);
            return;
        }
    };
    if let Err(e) = server::start(db_pool).await {
        eprintln!("Error starting server: {}", e);
    }   
    
}
