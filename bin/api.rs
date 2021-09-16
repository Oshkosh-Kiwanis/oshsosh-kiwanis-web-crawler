use actix_web::{get, Responder, web, App, HttpServer};
use actix_cors::Cors;

use log::info;

#[get("/goals")]
async fn get_goals(_path: web::Path<()>) -> impl Responder {
    info!("handling goals;");
    std::fs::read_to_string("contest-goals.json").unwrap_or("".into())
}

#[get("/dogs")]
async fn get_dogs(_path: web::Path<()>) -> impl Responder {
    info!("handling dogs;");
    std::fs::read_to_string("top-dogs.json").unwrap_or("".into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let addr = "0.0.0.0:8080";
    info!("started server; addr={}", addr);
    HttpServer::new(
        || {
            let cors = Cors::permissive();

            App::new()
                .wrap(cors)
                .service(get_goals)
                .service(get_dogs)
        }
    )
        .bind(addr)?
        .run()
        .await
}
