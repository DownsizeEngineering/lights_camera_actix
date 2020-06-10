use actix::prelude::*;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, Result};
use actix_files as fs;
use listenfd::ListenFd;
use std::sync::{Mutex};
use serde::Deserialize;
use actix_postgres::{bb8_postgres::tokio_postgres::tls::NoTls,
    PostgresActor, PostgresMessage
};

mod request_count;

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
                web::scope("/app").route("/index.html", web::get().to(index))
                .service(index3),
            )
            .configure(request_count::config_request_count)
            .route("hello/{name}/{greeting}", web::get().to(url_parser))
            .route("names", web::get().to(names))
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

struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
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

#[derive(Deserialize)]
struct Greeting {
    name: String,
    greeting: String
}


async fn url_parser(info: web::Path<Greeting>) -> Result<String> {
    Ok(format!("{}, {}!", info.greeting, info.name))
}
/* 
struct User {
    name: String,
}

impl Message for User {
    type Result = Result<User, Error>;
}
 */
#[derive(Deserialize)]
struct DBCredentials {
    address: String,
    name: String,
    password: String,
}

async fn pg () -> String{
    let db_cred: DBCredentials = serde_json::from_str(
        &std::fs::read_to_string("./secrets.json").unwrap()
    ).unwrap();

    let db_url = format!("postgresql://{}:{}@{}", 
        db_cred.name, db_cred.password, db_cred.address);

    let pg_actor = PostgresActor::start(&db_url, NoTls).unwrap(); 
    let task: PostgresMessage<_, NoTls, _> = PostgresMessage::new(|pool| {
        Box::pin(async move {
            let connection = pool.get().await?;
            connection
                .query("SELECT * FROM names", &vec![])
                .await
                .map_err(|err| err.into())
        })
    });
    let res = pg_actor.send(task).await.unwrap().unwrap();
    let val:&str = res[0].get(0);
    val.to_string()
}

async fn names() -> impl Responder {
    let val = pg().await;
    println!("pg got {}", val);
    HttpResponse::Ok().body("names")
}