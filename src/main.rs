use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};
use listenfd::ListenFd;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new().service(
            web::scope("/app").route("/index.html", web::get().to(index)),
        )
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8888")?
    };
    server.run().await
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