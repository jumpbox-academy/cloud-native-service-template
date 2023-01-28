use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::env;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hi!, this is Jumpbox team")
}


async fn jumpbox() -> impl Responder {
    HttpResponse::Ok().body("Tech Passion | Sharing | Society")
}

async fn env() -> impl Responder {
    let env = env::var("JUMPBOX").ok().unwrap();
    let output = format!("Jumpbox we are {}", env);
    HttpResponse::Ok().body(output)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .route("/jumpbox", web::get().to(jumpbox))
            .route("/env", web::get().to(env))
    })
    .bind(("0.0.0.0", 2001))?
    .run()
    .await
}