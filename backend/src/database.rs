use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::{MySqlPool, Error};
use sqlx::mysql::MySqlPoolOptions;

const DATABASE_POOL_SIZE: u32 = 10;

fn load_env() -> Result<String, std::env::VarError> {
    dotenv().ok();
    env::var("DATABASE_URL")
}

pub async fn get_database_connection_pool() -> Result<Arc<Mutex<MySqlPool>>, Error> {
    let db_url = match load_env() {
        Ok(url) => url,
        Err(e) => {
            return Err(Error::Configuration(Box::new(e) as Box<dyn std::error::Error + Send + Sync>));
        }
    };

    let pool = MySqlPoolOptions::new()
        .max_connections(DATABASE_POOL_SIZE)
        .connect(&db_url)
        .await?;
    Ok(Arc::new(Mutex::new(pool)))
}

mod test {
    use std::env;
    use dotenvy::dotenv;
    
    #[tokio::test]
    async fn database_connection() {
        let pool = super::get_database_connection_pool().await.unwrap();
        let pool = pool.lock().await;
        assert!(pool.acquire().await.is_ok());
        
    }
}