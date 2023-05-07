use actix_web::web;

use super::controller::{health_checker_handler, todos_list_handler, create_todo_handler,get_todo_handler,edit_todo_handler,delete_todo_handler};

pub fn todo_config() -> actix_web::Scope {
    let scope = web::scope("/todo")
        .service(health_checker_handler)
        .service(todos_list_handler)
        .service(create_todo_handler)
        .service(get_todo_handler)
        .service(edit_todo_handler)
        .service(delete_todo_handler);

    scope
}
