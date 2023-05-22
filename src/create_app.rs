use crate::api::controllers::authentication_handler::{login_handler, register_handler};
use crate::api::controllers::todo_handler::{
    create_todo_handler, delete_todo_handler, get_todo_handler, list_todos_handler,
};
use crate::api::middleware::ServiceContextMaintenanceCheck;
use crate::container::Container;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::{web, App};
use actix_web::{Error, HttpResponse};

pub fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let container = Container::new();
    let todo_service = container.todo_service.clone();
    let authentication_service = container.authentication_service.clone();
    let service_context_service = container.service_context_service.clone();

    App::new()
        .app_data(web::Data::from(todo_service.clone()))
        .app_data(web::Data::from(authentication_service.clone()))
        .app_data(web::Data::from(service_context_service.clone()))
        .wrap(Logger::new("%a %{User-Agent}i"))
        .wrap(ServiceContextMaintenanceCheck)
        .route("/health", web::get().to(HttpResponse::Ok))
        .service(
            web::scope("/todos")
                .route("", web::post().to(create_todo_handler))
                .route("", web::get().to(list_todos_handler))
                .route("/{id}", web::get().to(get_todo_handler))
                .route("/{id}", web::delete().to(delete_todo_handler)),
        )
        .service(
            web::scope("/auth")
                .route("/login", web::post().to(login_handler))
                .route("/register", web::post().to(register_handler)),
        )
}
