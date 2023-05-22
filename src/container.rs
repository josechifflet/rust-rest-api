use crate::core::db::postgresql::db_pool;
use crate::core::repositories::todo::TodoDieselRepository;
use crate::core::repositories::user::UserDieselRepository;
use crate::core::services::service_context::ServiceContextServiceImpl;
use crate::domain::repositories::todo::TodoRepository;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::authentication::AuthenticationService;
use crate::domain::services::service_context::ServiceContextService;
use crate::domain::services::todo::TodoService;
use crate::domain::services::user::UserService;
use crate::services::authentication::AuthenticationServiceImpl;
use crate::services::todo::TodoServiceImpl;
use crate::services::user::UserServiceImpl;
use std::sync::Arc;

pub struct Container {
    pub todo_service: Arc<dyn TodoService>,
    pub user_service: Arc<dyn UserService>,
    pub authentication_service: Arc<dyn AuthenticationService>,
    pub service_context_service: Arc<dyn ServiceContextService>,
}

impl Container {
    pub fn new() -> Self {
        let todo_repository: Arc<dyn TodoRepository> =
            Arc::new(TodoDieselRepository::new(Arc::new(db_pool())));
        let todo_service = Arc::new(TodoServiceImpl {
            repository: todo_repository,
        });
        let user_repository: Arc<dyn UserRepository> =
            Arc::new(UserDieselRepository::new(Arc::new(db_pool())));
        let user_service = Arc::new(UserServiceImpl {
            repository: user_repository,
        });
        let user_repository_for_auth: Arc<dyn UserRepository> =
            Arc::new(UserDieselRepository::new(Arc::new(db_pool())));
        let authentication_service = Arc::new(AuthenticationServiceImpl {
            user_repository: user_repository_for_auth,
        });
        let service_context_service = Arc::new(ServiceContextServiceImpl::new(Arc::new(db_pool())));
        Container {
            todo_service,
            user_service,
            authentication_service,
            service_context_service,
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
