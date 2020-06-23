use actix_web::{web, HttpResponse, Responder};
use super::{pg_query, PGA};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TodoList {
    id: i32,
    name: String,
    tasks: Vec<Todo>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Todo {
        id: i32,
        task: String,
        details: Option<String>,
        completed: bool,
}

pub async fn get_all_lists(db: web::Data<PGA>) -> impl Responder {
    let rows = pg_query(db.get_ref(), "SELECT * FROM done.lists").await;
    
    let mut lists: Vec<TodoList> = Vec::new();
    for row in rows {
        lists.push(TodoList {
            id: row.get(0),
            name: row.get(1),
            tasks: Vec::new(),
        });
    }
    
    let rows = pg_query(db.get_ref(), "SELECT * FROM done.todos").await;
    for row in rows {
        let list:i32 = row.get(4);
        let list = list as usize - 1;
        lists[list].tasks.push(Todo {
            id: row.get(0),
            task: row.get(1),
            details: match row.try_get(2) {
                Err(_) => None,
                Ok(x) => x
            },
            completed: row.get(3),
        });
    }

    web::Json(lists)
}

pub async fn new_list(db: web::Data<PGA>, list: TodoList) -> impl Responder {
    let res = pg_query(db.get_ref(), 
        &format!("INSERT INTO done.lists (name) VALUES ('{}') RETURNING (id);", 
        list.name)
    ).await;
    let id: i32 = res[0].get(0);
    HttpResponse::Ok().body(format!("{}", id))
}

pub async fn new_todo(
    db: web::Data<PGA>,
    todo: Todo, 
    list_id: u32
) -> impl Responder {
    let res = match todo.details {
        Some(details) => {
            pg_query(db.get_ref(),
                &format!("INSERT INTO done.todos (task, details, completed, list) 
                    VALUES ('{}', '{}', {}, {}) RETURNING (id);", 
                    todo.task, 
                    details,
                    todo.completed, list_id
                )
            ).await
        },
        None => {
            pg_query(db.get_ref(),
                &format!("INSERT INTO done.todos (task, completed, list) 
                    VALUES ('{}', {}, {}) RETURNING (id);", 
                    todo.task,
                    todo.completed, list_id
                )
            ).await
        }
    };
    
    let id: i32 = res[0].get(0);
    HttpResponse::Ok().body(format!("{}", id))
}