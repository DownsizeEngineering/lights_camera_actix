use actix_web::{web, App, HttpServer};
use actix_files as fs;
use listenfd::ListenFd;
use std::sync::{Mutex};

mod request_count;
mod endpoints;
mod db;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    
    let counter = web::Data::new(AppStateWithCounter{
        counter: Mutex::new(0),
    });
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .data(AppState{
                app_name: String::from("lights_camera_actix"),
            })
            .service(
                web::scope("/app").route("/index.html", web::get().
                to(endpoints::index))
                .service(endpoints::index3),
            )
            .configure(request_count::config_request_count)
            .route("hello/{name}/{greeting}", web::get().
                to(endpoints::url_parser))
            .route("names", web::get().to(db::names))

            .service(fs::Files::new("/", "./client/public")
                .index_file("index.html")
            )
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8888")?
    };
    server.run().await
}

pub struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}
