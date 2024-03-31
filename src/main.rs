mod handlers;
mod utils;
mod utils_test;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::utils::get_attrs;

#[get("/")]
async fn hello() -> impl Responder {
    println!("hello");
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let path = "/mnt/d/Code/rust/spatial-import-server/data/beijing.shp";
    get_attrs(path);
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}