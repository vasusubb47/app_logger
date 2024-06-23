use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow, Pool, Postgres};

#[derive(Debug, Serialize)]
pub enum DbError {
    ConnectionError(String),
    MigrationsError(String),
    QueryError(String),
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct RowCount {
    pub count: i64,
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
