use actix_web::{get, middleware::Logger, web::{self, Data}, App, HttpServer};
use controllers::{organizations::organization_config, users::user_config};
use dotenv::{dotenv, var};
use crate::database::pg_db_connection;

mod database;
mod app_data;
mod models;
mod controllers;
mod utils;

#[get("/")]
async fn index() -> web::Json<String> {
    web::Json("hello world!!".to_owned())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let db_url = var("DATABASE_URL").expect("Couldn't find database url from environment variable.");

    let app_data = app_data::AppData {
        pg_conn: pg_db_connection(&db_url).await.expect(""),
    };

    HttpServer::new(move || {
        App::new()
        .app_data(Data::new(app_data.clone()))
        .service(
            web::scope("/api")
            .configure(user_config)
            .configure(organization_config),
        ).wrap(Logger::default())
    })
    .bind(("127.0.0.1", 6969))?
    .run()
    .await
}
