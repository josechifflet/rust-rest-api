## Architecture Overview

The onion architecture is a layered architecture that is based on the onion model.
Where each layer in the onion model is used to define the different layers of an application.

For this rust implementation 4 layers are used.

- api (app) module: The outermost layer that contains the controllers and the endpoints definition, serialization and deserialization of the data, validation and error handling.
- core: Layer that typically include database connections, external APIs calls, logging and core configuration management.
- services: Layer that contains the application's services, which encapsulate the core business logic and provide a higher-level abstraction for the application to interact with the domain entities.
- domain: The innermost layer that contains the core business logic and entities of the application.

Folder structure:

```
.
├── migrations
├── scripts
│   └── run_postgres.sh # Run postgres in docker locally
├── src
│   ├── api
│   │   ├── controllers
│   │   │   └── ...  # controllers for the api
│   │   ├── dto # Data transfer objects
│   │   │   └── ... # Individual DTOs
│   │   └── errors.py
│   ├── core
│   │   ├── services
│   │   │   └── ...  # Services that use third party libraries or services (e.g. email service)
│   │   ├── databases
│   │   │   └── ...  # Database adapaters and initialization
│   │   ├── repositories
│   │   │   └── ...  # Repositories for interacting with the databases
│   │   └── models
│   │       └── ...  # Database models
│   ├── domain
│   │   ├── mod.rs
│   │   ├── constants.rs
│   │   ├── errors.rs
│   │   ├── models
│   │   │   └── ...  # Business logic models traits or structs
│   │   ├── services
│   │   │   └── ...  # Service traits
│   │   └── repositories
│   │       └── ...  # Repository traits
│   ├── services
│   │   └── ...  # Concrete service implementation for interacting with the domain (business logic)
│   ├── container.rs
│   ├── create_app.rs # app factory
│   ├── lib.rs
│   └── main.rs
```

- migrations: Alembic's migration scripts are stored here.
- scripts: contains the application's configuration settings.
