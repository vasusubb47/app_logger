use actix_web::{get, web, HttpResponse, Responder};

use crate::{app_data::AppData, models::organizations::get_all_organizations_db};


pub fn organization_config(config: &mut web::ServiceConfig) {
    let scope = web::scope("/org")
        .service(get_all_organizations);
    // .service(user_login);

    config.service(scope);
}

#[get("/")]
pub async fn get_all_organizations(data: web::Data<AppData>) -> impl Responder {
    let organizations = get_all_organizations_db(&data.pg_conn).await;

    match organizations {
        Ok(orgs) => {
            HttpResponse::Ok().json(web::Json(orgs))
        },
        Err(e) => {
            println!("Error: {:#?}", e);
            HttpResponse::InternalServerError().json(web::Json(e))
        }
    }
}

