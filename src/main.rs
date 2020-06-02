use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};
use listenfd::ListenFd;
use std::sync::{Mutex};

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .data(AppState{
                app_name: String::from("lights_camera_actix"),
            })
            .service(
                web::scope("/app").route("/index.html", web::get().to(index))
                .service(index3),
            )
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8888")?
    };
    server.run().await
}

struct AppState {
    app_name: String,
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hello {}!", app_name))
}
async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello Universe!")
}

#[get("/hi")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hi user!")
}