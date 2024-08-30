//use actix_web::{web, App, HttpServer, Responder, HttpResponse};
//use mongodb::{Client, options::ClientOptions, Database, bson::doc};
//use bcrypt::{DEFAULT_COST, hash, verify};
use pico_args::{Arguments, Error};
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;

mod models;
mod server;
mod database;


const HELP_MESSAGE : &str = r#"
FLAGS:
-h, --help : Print this help message
-l, --logs : Enable logging into stdout

ARGUMENTS:
--key <[]> :                    The API key for the AI service (REQUIRED)
--ip-address <[].[].[].[]> :    The ip address to bind the server to. Default is 127.0.0.1
--port <[]> :                   The port to bind the server to. Default is 8080
--db-pool-size <[]> :           The size of the database connection pool. Default is 10
"#;

#[tokio::main]
async fn main() {
    let args : AppArgs = match parse_args() {
        Ok(Some(args)) => args,
        Ok(None) => return,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            return;
        }
    };
    
    if let Some(key) = args.api_key.as_deref() {
        dbg!(&key);
        std::env::set_var("DUOLINGO_APP_API_KEY", key);
        dbg!(std::env::var("DUOLINGO_APP_API_KEY"));
    }
    
    let db_pool = match database::get_database_connection_pool(args.db_pool_size).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Error connecting to database: {}", e);
            return;
        }
    };
    if let Err(e) = server::start(db_pool, args.ip_address.as_deref(), args.port).await {
        eprintln!("Error starting server: {}", e);
    }
}

#[derive(Debug)]
struct AppArgs {
    ip_address : Option<String>,
    port : Option<u32>,
    db_pool_size : Option<u32>,
    api_key: Option<String>
}

fn parse_args() -> Result<Option<AppArgs>, Error> {
    let mut p_args = Arguments::from_env();
    
    if p_args.contains(["-h", "--help"]) {
        println!("{}", HELP_MESSAGE);
        return Ok(None);
    }
    
    if p_args.contains(["-l", "--logs"]) {
        let subscriber = tracing_subscriber::fmt()
            .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
            .finish();
        tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
        
        info!("logging enabled");
    }
    
    let args = AppArgs {
        ip_address : p_args.opt_value_from_str("--ip-address")?,
        port : p_args.opt_value_from_str("--port")?,
        db_pool_size : p_args.opt_value_from_str("--db-pool-size")?,
        api_key : p_args.opt_value_from_str("--key")?
    };
    
    let remaining = p_args.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }
    
    Ok(Some(args))
}
