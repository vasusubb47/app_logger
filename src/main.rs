use actix_web::{get, web, App, HttpServer};
use dotenv::{dotenv, var};

use crate::database::pg_db_connection;

mod database;
mod app_data;

#[get("/")]
async fn index() -> web::Json<String> {
    web::Json("hello world!!".to_owned())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = var("DATABASE_URL").expect("Couldn't find database url from environment variable.");

    let app_data = app_data::AppData {
        pg_conn: pg_db_connection(&db_url).await.expect(""),
    };

    HttpServer::new(move || {
        App::new()
        .app_data(app_data.clone())
        .service(
            web::scope("/api").service(index)
        )
    })
    .bind(("127.0.0.1", 6969))?
    .run()
    .await
}
