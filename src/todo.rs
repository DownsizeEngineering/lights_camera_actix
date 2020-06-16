use actix_web::web;

struct TodoList {
    name: String,
    tasks: Vec<Todo>,
}

struct Todo {
    task: String,
    completed: bool,
}
