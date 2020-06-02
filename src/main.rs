use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};
#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/universe", web::get().to(index2))
            .service(index3)
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}
async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello Universe!")
}

#[get("/hi")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hi user!")
}