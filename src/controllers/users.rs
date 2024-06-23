use actix_web::{delete, get, post, web, HttpResponse, Responder};

use crate::{app_data::AppData, models::users::{delete_user_by_id, get_all_users_db, insert_user_db, DeleteUser, NewUser, UserError}};

pub fn user_config(config: &mut web::ServiceConfig) {
    let scope = web::scope("/user")
        .service(get_all_users)
        .service(register_user)
        .service(delete_user);
    // .service(user_login);

    config.service(scope);
}

#[get("/")]
pub async fn get_all_users(data: web::Data<AppData>) -> impl Responder {
    let users = get_all_users_db(&data.pg_conn).await;

    match users {
        Ok(users) => {
            HttpResponse::Ok().json(web::Json(users))
        },
        Err(e) => {
            println!("Error: {:#?}", e);
            HttpResponse::InternalServerError().json(web::Json(e))
        }
    }
}

#[post("/register")]
pub async fn register_user(data: web::Data<AppData>, user_detailes: web::Json<NewUser>) -> impl Responder {
    let new_user = insert_user_db(&data.pg_conn, user_detailes.0).await;

    match new_user {
        Ok(user) => {
            HttpResponse::Ok().json(web::Json(user))
        },
        Err(error) => {
            println!("Error: {:#?}", error);
            match error {
                UserError::UserNotFound(_) => HttpResponse::NotFound().json(web::Json(error)),
                UserError::EmailInUse(_) => HttpResponse::Forbidden().json(web::Json(error)),
                UserError::Error => HttpResponse::InternalServerError().json(web::Json(error)),
            }
        }
    }
}

#[delete("/")]
pub async fn delete_user(data: web::Data<AppData>, delete_user: web::Json<DeleteUser>) -> impl Responder {
    let delete_user = delete_user_by_id(&data.pg_conn, delete_user.0.user_id).await;

    match delete_user {
        Ok(user) => {
            HttpResponse::Ok().json(web::Json(user))
        },
        Err(error) => {
            println!("Error: {:#?}", error);
            match error {
                UserError::UserNotFound(_) => HttpResponse::NotFound().json(web::Json(error)),
                _ => HttpResponse::InternalServerError().json(web::Json(error)),
            }
        }
    }
}
