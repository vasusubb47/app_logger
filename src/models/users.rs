use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{database::{DbError, RowCount}, utils::genarate_salt};

// User definations

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub created_date: DateTime<Utc>,
    pub updated_date: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserLogin {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUser {
    pub user_id: Uuid,
}

// User Error definition

#[derive(Debug, Serialize)]
pub enum UserError {
    UserNotFound (String),
    EmailInUse (String),
    Error
}

// User functions

pub async fn get_all_users_db(pool: &PgPool) -> Result<Vec<User>, DbError> {
    let query = "\
        SELECT user_id, name, email, \
        created_at as created_date, updated_at as updated_date \
        FROM Users;";

    let query = sqlx::query_as::<_, User>(query);

    let users = query.fetch_all(pool).await;

    match users {
        Ok(users) => Ok(users),
        Err(e) => Err(DbError::QueryError(e.to_string())),
    }
}

pub async fn insert_user_db(pool: &PgPool, user_detailes: NewUser) -> Result<Vec<User>, UserError> {
    let email_allready_exists = does_email_exist_db(pool, user_detailes.email.to_owned()).await;

    match email_allready_exists {
        Ok(exists) => if exists {
            return Err(UserError::EmailInUse(format!("email {} already exists.", user_detailes.email.to_owned())));
        },
        Err(err) => return Err(err),
    }

    let mut sha = Sha256::new();
    let salt = genarate_salt(64);

    sha.update(user_detailes.password + &salt.to_owned());
    let passcode_hash = sha.finalize();
    let passcode_hash = format!("{:X}:{}", passcode_hash, salt);
    
    let query = "INSERT INTO \
        users (name, email, password) \
        VALUES ($1, $2, $3)\
        returning user_id";
    let query = sqlx::query(query)
        .bind(user_detailes.name)
        .bind(user_detailes.email.to_owned())
        .bind(passcode_hash)
        .execute(pool)
        .await;
    
    match query {
        Ok(_) => {
            get_user_by_email_db(pool, user_detailes.email).await
        },
        Err(err) => {
            println!("{}", err);
            Err(UserError::Error)
        },
    }
}

pub async fn get_user_by_email_db(pool: &PgPool, email: String) -> Result<Vec<User>, UserError> {
    
    let query = "\
        SELECT user_id, name, email, \
        created_at as created_date, updated_at as updated_date \
        FROM Users \
        WHERE email = $1";

    let query = sqlx::query_as::<_, User>(query).bind(email.to_owned());

    let users = query.fetch_all(pool).await;

    match users {
        Ok(users) => Ok(users),
        Err(e) => {
            println!("error getting user by email: \n email: {} \n error: {}", email, e);
            Err(UserError::Error)
        },
    }
}

pub async fn does_email_exist_db(pool: &PgPool, email: String) -> Result<bool, UserError> {
    let query = "\
        SELECT count(*) \
        FROM Users \
        WHERE email = $1";

    let query = sqlx::query_as::<_, RowCount>(query).bind(email.to_owned());

    let row_count = query.fetch_one(pool).await;

    match row_count {
        Ok(row_count) => Ok(row_count.count >= 1),
        Err(e) => {
            println!("error checking user by email: \n email: {} \n error: {}", email, e);
            Err(UserError::Error)
        },
    }
}

pub async fn does_user_id_exist_db(pool: &PgPool, user_id: Uuid) -> Result<bool, UserError> {
    let query = "\
        SELECT count(*) \
        FROM Users \
        WHERE user_id = $1";

    let query = sqlx::query_as::<_, RowCount>(query).bind(user_id);

    let row_count = query.fetch_one(pool).await;

    match row_count {
        Ok(row_count) => Ok(row_count.count >= 1),
        Err(e) => {
            println!("error checking user by user_id: \n user_id: {} \n error: {}", user_id, e);
            Err(UserError::Error)
        },
    }}

pub async fn delete_user_by_id(pool: &PgPool, id: Uuid) -> Result<bool, UserError> {

    let user_exists = does_user_id_exist_db(pool, id).await;
    match user_exists { 
        Ok(exists) => if!exists {
            return Err(UserError::UserNotFound(format!("user by user_id: {}, dosen't exists", id)));
        },
        Err(err) => return Err(err),
    }

    let query = "\
        DELETE FROM \
        Users \
        WHERE user_id = $1";

    let query = sqlx::query(query).bind(id);

    let row_count = query.execute(pool).await;

    match row_count {
        Ok(row_count) => Ok(row_count.rows_affected() == 1),
        Err(e) => {
            println!("error deleting user by user_id: \n user_id: {} \n error: {}", id, e);
            Err(UserError::Error)
        },
    }
}
