use actix_web::{web, Responder};
use super::{pg_query, PGA};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct TodoList {
    id: i32,
    name: String,
    tasks: Vec<Todo>,
}

#[derive(Serialize, Debug)]
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