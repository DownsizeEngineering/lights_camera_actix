use actix_web::{web, Responder, HttpResponse};
use crate::db;

// struct TodoList {
//     name: String,
//     tasks: Vec<Todo>,
// }

// struct Todo {
    //     task: String,
    //     description: String,
    //     completed: bool,
// }


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/list")
            .route("", web::post().to(new_list))
            .route("", web::get().to(get_all_lists))
            .service(web::scope("/{list_id}")
                .route("", web::post().to(new_todo))
                .route("", web::get().to(get_list))
                // .route("", web::delete().to(delete_list))
                // .route("", web::patch().to(rename_list))
            )
    )
    .service(
        web::resource("/todo/{todo_id}")
            .route(web::get().to(get_todo))
            // .route(web::patch().to(update_todo))
            // .route(web::delete().to(delete_todo))
    );
}

async fn new_list() -> impl Responder {
    HttpResponse::Ok().body("new list")
}

async fn get_all_lists() -> impl Responder {
    HttpResponse::Ok().body("get all lists")
}


async fn new_todo(info: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("new todo for list {}", info))
}

async fn get_list(info: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("get list {}", info))
}

async fn get_todo(info: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("get todo {}", info))
}