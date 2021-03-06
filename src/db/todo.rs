use actix_web::{web, HttpResponse, Responder};
use super::{pg_query, PGA};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TodoList {
    id: i32,
    name: String,
    tasks: Vec<Todo>,
}

impl TodoList {
    pub fn new (name: String) -> Self {
        TodoList {
            id: 0,
            name,
            tasks: Vec::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Todo {
        id: i32,
        task: String,
        details: Option<String>,
        completed: bool,
}

pub async fn get_all_lists(db: web::Data<PGA>) -> impl Responder {
    let rows = pg_query(db.get_ref(), "SELECT * FROM done.lists;").await;
    
    let mut lists: HashMap<i32, TodoList> = HashMap::new();
    for row in rows {
        let list = TodoList {
            id: row.get(0),
            name: row.get(1),
            tasks: Vec::new(),
        };
        lists.insert(list.id, list);
    }
    
    let rows = pg_query(db.get_ref(), 
        "SELECT * FROM done.todos ORDER BY id ASC;"
    ).await;
    for row in rows {
        let list:i32 = row.get(4);
        lists.get_mut(&list).unwrap().tasks.push(Todo {
            id: row.get(0),
            task: row.get(1),
            details: match row.try_get(2) {
                Err(_) => None,
                Ok(x) => x
            },
            completed: row.get(3),
        });
    }

    let mut list_indexes: Vec<i32> = Vec::new();
    for list in lists.iter() {
        list_indexes.push(list.0.clone());
    }
    list_indexes.sort();

    let mut lists_vec: Vec<TodoList> = Vec::new();
    for index in list_indexes {
        lists_vec.push(lists.get(&index).unwrap().clone());
    }

    web::Json(lists_vec)
}

pub async fn new_list(db: web::Data<PGA>, list: TodoList) -> impl Responder {
    let res = pg_query(db.get_ref(), 
        &format!("INSERT INTO done.lists (name) VALUES (E'{}') RETURNING (id);", 
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
                    VALUES (E'{}', E'{}', {}, {}) RETURNING (id);", 
                    todo.task, 
                    details,
                    todo.completed, list_id
                )
            ).await
        },
        None => {
            pg_query(db.get_ref(),
                &format!("INSERT INTO done.todos (task, completed, list) 
                    VALUES (E'{}', {}, {}) RETURNING (id);", 
                    todo.task,
                    todo.completed, list_id
                )
            ).await
        }
    };
    
    let id: i32 = res[0].get(0);
    HttpResponse::Ok().body(format!("{}", id))
}

pub async fn update_complete(
    db: web::Data<PGA>, 
    todo_id: u32, 
    status: bool
) -> impl Responder {
    pg_query(db.get_ref(), 
        &format!("UPDATE done.todos SET completed={} WHERE id={}",
        status, todo_id)).await;
    HttpResponse::Ok().body(format!("{}", status))
}

pub async fn delete_todo(db: web::Data<PGA>, todo_id: u32) -> impl Responder{
    pg_query(db.get_ref(),
    &format!("DELETE FROM done.todos WHERE (id = {});", todo_id)
    ).await;
    HttpResponse::Ok().body("")
}

pub async fn delete_list(db: web::Data<PGA>, list_id: u32) -> impl Responder{
    pg_query(db.get_ref(),
    &format!("DELETE FROM done.lists WHERE (id = {});", list_id)
    ).await;
    HttpResponse::Ok().body("")
}