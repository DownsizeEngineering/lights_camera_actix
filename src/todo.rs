use actix_web::{web, Responder, HttpResponse};
use crate::db::{todo, PGA, todo::TodoList, todo::Todo};
use serde::Deserialize;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/list")
            .route("", web::post().to(new_list))
            .route("", web::get().to(get_all_lists))
            .service(web::scope("/{list_id}")
                .route("", web::post().to(new_todo))
                .route("", web::get().to(get_list))
                .route("", web::delete().to(delete_list))
                // .route("", web::patch().to(rename_list))
            )
    )
    .service(
        web::resource("/todo/{todo_id}")
            .route(web::get().to(get_todo))
            .route(web::patch().to(update_complete))
            .route(web::delete().to(delete_todo))
    );
}

#[derive(Deserialize)]
struct NewTodoList {
    name: String,
}

async fn new_list(db: web::Data<PGA>, form: web::Query<NewTodoList>
) -> impl Responder {
    let new_list = TodoList::new(form.into_inner().name);
    todo::new_list(db, new_list).await
}

async fn get_all_lists(db: web::Data<PGA>) -> impl Responder {
    todo::get_all_lists(db).await
}


async fn new_todo(
    db: web::Data<PGA>, 
    todo: web::Query<Todo>,
    list_id: web::Path<u32>
) -> impl Responder {
    todo::new_todo(db, todo.into_inner(), *list_id).await
}

async fn get_list(info: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("get list {}", info))
}

async fn get_todo(info: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("get todo {}", info))
}

#[derive(Deserialize)]
struct Status {
    status: bool,
}
async fn update_complete(
    db: web::Data<PGA>,
    todo_id: web::Path<u32>, 
    status: web::Query<Status>
) -> impl Responder {
    todo::update_complete(db, *todo_id, status.into_inner().status).await
}

async fn delete_todo(
    db: web::Data<PGA>,
    todo_id: web::Path<u32>
) ->impl Responder {
    todo::delete_todo(db, *todo_id).await
}

async fn delete_list(
    db: web::Data<PGA>,
    list_id: web::Path<u32>
) ->impl Responder {
    todo::delete_list(db, *list_id).await
}