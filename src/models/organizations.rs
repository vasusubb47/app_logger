use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::database::DbError;

// Organization definations

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Organization {
    org_id: Uuid,
    name: String,
    created_date: DateTime<Utc>,
    updated_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewOrganization {
    name: String,
    creater_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserAccessStatus {
    Admin,
    Member,
    Guest,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserOrganization {
    user_id: Uuid,
    org_id: Uuid,
    access_status: UserAccessStatus,
    created_date: DateTime<Utc>,
    updated_date: Option<DateTime<Utc>>,
}

// Organization methods

pub async fn get_all_organizations_db(pool: &PgPool) -> Result<Vec<Organization>, DbError> {
    let query = "\
        SELECT org_id, name, \
        created_at as created_date, updated_at as updated_date \
        FROM Organization;";

    let query = sqlx::query_as::<_, Organization>(query);

    let ogrinations = query.fetch_all(pool).await;

    match ogrinations {
        Ok(ogrinations) => Ok(ogrinations),
        Err(e) => Err(DbError::QueryError(e.to_string())),
    }
}

pub async fn get_organization_by_name_db(pool: &PgPool, name: String) -> Result<Organization, DbError> {
    let query = "\
        SELECT org_id, name, \
        created_at as created_date, updated_at as updated_date \
        FROM Organization \
        WHERE name = $1;";
    
    let query = sqlx::query_as::<_, Organization>(query).bind(name);
    
    let ogrination = query.fetch_one(pool).await;
    
    match ogrination {
        Ok(ogrination) => Ok(ogrination),
        Err(e) => Err(DbError::QueryError(e.to_string())),
    }
}

pub async fn insert_new_organization(pool: &PgPool, new_org: NewOrganization) -> Result<Organization, DbError> {
    let query = "\
        INSERT INTO Organization (name) \
        VALUES ($1) \
        RETURNING org_id, name, \
        created_at as created_date, updated_at as updated_date;";
        
    let query = sqlx::query_as::<_, Organization>(query).bind(new_org.name);

    let ogrination = query.fetch_one(pool).await;

    match ogrination {
        Ok(ogrination) => Ok(ogrination),
        Err(e) => Err(DbError::QueryError(e.to_string())),
    }
}
