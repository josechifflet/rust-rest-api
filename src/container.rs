use std::sync::Arc;
use crate::core::repositories::user::UserDieselRepository;
use crate::domain::repositories::todo::TodoRepository;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::service_context::ServiceContextService;
use crate::domain::services::todo::TodoService;
use crate::core::db::postgresql::db_pool;
use crate::core::repositories::todo::TodoDieselRepository;
use crate::core::services::service_context::ServiceContextServiceImpl;
use crate::domain::services::user::UserService;
use crate::services::todo::TodoServiceImpl;
use crate::services::user::UserServiceImpl;

pub struct Container {
    pub todo_service: Arc<dyn TodoService>,
    pub user_service: Arc<dyn UserService>,
    pub service_context_service: Arc<dyn ServiceContextService>
}

impl Container {
    pub fn new() -> Self {
        let todo_repository: Arc<dyn TodoRepository> = Arc::new(
            TodoDieselRepository::new(Arc::new(db_pool()))
        );
        let todo_service = Arc::new(
            TodoServiceImpl { repository: todo_repository }
        );
        let user_repository: Arc<dyn UserRepository> = Arc::new(
            UserDieselRepository::new(Arc::new(db_pool()))
        );
        let user_service = Arc::new(
            UserServiceImpl { repository: user_repository }
        );      
        let service_context_service = Arc::new(
            ServiceContextServiceImpl::new(Arc::new(db_pool()))
        );
        Container { todo_service, user_service, service_context_service }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}