use sqlx::{self, Pool, Postgres};

#[derive(Debug)]
pub enum DbError {
    ConnectionError(String),
    MigrationsError(String),
}

pub async fn pg_db_connection(url: &str) -> Result<Pool<Postgres>, DbError> {
    let pool = match Pool::connect(url).await {
        Ok(pool) => pool,
        Err(e) => return Err(DbError::ConnectionError(e.to_string()))
    };

    match sqlx::migrate!("./database/migrations").run(&pool).await {
        Ok(_) => Ok(pool),
        Err(e) => return Err(DbError::MigrationsError(e.to_string()))
    }
}