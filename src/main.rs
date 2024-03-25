use actix_web::{web, get, App, HttpServer, Responder, HttpResponse};


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]

async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
    }).bind(("127.0.0.1", 8080))?.run().await
}
