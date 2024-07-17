use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::{MySqlPool, Error};

fn load_env() -> Result<String, std::env::VarError> {
    dotenv().ok();
    env::var("DATABASE_URL")
}

async fn get_database_connection_pool() -> Result<Arc<Mutex<MySqlPool>>, Error> {
    let db_url = match load_env() {
        Ok(url) => url,
        Err(e) => {
            return Err(Error::Configuration(Box::new(e) as Box<dyn std::error::Error + Send + Sync>));
        }
    };

    let pool = MySqlPool::connect(&db_url).await?;
    Ok(Arc::new(Mutex::new(pool)))
}

pub async fn initialize_db_pool() -> Arc<Mutex<sqlx::MySqlPool>> {
    let mut db_pool : Option<Arc<Mutex<MySqlPool>>> = None;

    for i in 0..10 {
        match get_database_connection_pool().await {
            Ok(pool) => {
                db_pool = Some(pool);
                break;
            }

            Err(e) => {
                if i == 9 {
                    panic!("{}", e);
                }
                continue;
            }
        }
    }
    db_pool.unwrap()
}

mod test {

    #[tokio::test]
    async fn database_connection() {
        let pool = super::get_database_connection_pool().await.unwrap();
        let pool = pool.lock().await;
        assert!(pool.acquire().await.is_ok());
    }

}