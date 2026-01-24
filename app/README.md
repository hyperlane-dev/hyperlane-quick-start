<center>

## hyperlane-app

<img src="https://docs.ltpp.vip/img/hyperlane.png" alt="" height="160">

[![](https://img.shields.io/crates/v/hyperlane.svg)](https://crates.io/crates/hyperlane)
[![](https://img.shields.io/crates/d/hyperlane.svg)](https://img.shields.io/crates/d/hyperlane.svg)
[![](https://docs.rs/hyperlane/badge.svg)](https://docs.rs/hyperlane)
[![](https://github.com/hyperlane-dev/hyperlane/workflows/Rust/badge.svg)](https://github.com/hyperlane-dev/hyperlane/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane.svg)](./license)

</center>

> Hyperlane application module containing core application logic, controllers, services, and middleware components.

## Overview

The `hyperlane-app` module is the main application layer of the hyperlane framework. It implements the application logic following domain-driven design principles and provides controllers, services, and middleware components.

## Directory Structure

```
app/
├── aspect/            # AOP aspects
├── controller/        # Request handlers
├── domain/document/   # Domain models - Document
├── exception/         # Exception handling
│   ├── application/   # Application exceptions
│   └── framework/     # Framework exceptions
├── filter/            # Request/response filters
├── mapper/            # Data mappers
│   ├── dataset/
│   └── document/
├── middleware/        # Middleware components
│   ├── request/
│   └── response/
├── model/             # Data models
│   ├── application/
│   ├── data_transfer/
│   ├── param/
│   └── user_model/
├── service/           # Business logic services
├── utils/             # Utility functions
└── Cargo.toml
```

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      hyperlane-app                               │
├─────────────────────────────────────────────────────────────────┤
│  Controller Layer                                               │
│  ├── Handles HTTP requests                                      │
│  ├── Validates input                                            │
│  └── Returns responses                                          │
├─────────────────────────────────────────────────────────────────┤
│  Service Layer                                                  │
│  ├── Business logic implementation                              │
│  ├── Transaction management                                     │
│  └── Service composition                                        │
├─────────────────────────────────────────────────────────────────┤
│  Domain Layer                                                   │
│  ├── Domain models                                              │
│  ├── Value objects                                              │
│  └── Domain events                                              │
├─────────────────────────────────────────────────────────────────┤
│  Middleware Layer                                               │
│  ├── Request preprocessing                                      │
│  ├── Response postprocessing                                    │
│  └── Cross-cutting concerns                                     │
└─────────────────────────────────────────────────────────────────┘
```

## Features

- **Controller Pattern**: Structured request handling with derive macros
- **Service Layer**: Business logic isolation and composition
- **Middleware Stack**: Composable middleware for request processing
- **Exception Handling**: Centralized exception management
- **Data Mapping**: Clean separation between layers
- **Filter Chain**: Request/response filtering

## Quick Start

### Creating a Controller

```rust
use hyperlane_app::{controller, Response, Json};

#[controller("/api/users")]
pub struct UserController;

impl UserController {
    #[get("/{id}")]
    async fn get_user(id: u64) -> Result<Json<UserResponse>, AppError> {
        let user = UserService::find_by_id(id).await?;
        Ok(Json(user.into()))
    }

    #[post("/")]
    async fn create_user(Json(req): Json<CreateUserRequest>) -> Result<Json<UserResponse>, AppError> {
        let user = UserService::create(&req).await?;
        Ok(Json(user.into()))
    }
}
```

### Creating a Service

```rust
use hyperlane_app::service;

#[service]
pub trait UserService {
    async fn find_by_id(id: u64) -> Result<User, AppError>;
    async fn create(req: &CreateUserRequest) -> Result<User, AppError>;
    async fn update(id: u64, req: &UpdateUserRequest) -> Result<User, AppError>;
    async fn delete(id: u64) -> Result<(), AppError>;
}
```

### Middleware Usage

```rust
use hyperlane_app::middleware::RequestMiddleware;

#[derive(Clone)]
struct AuthMiddleware;

#[async_trait::async_trait]
impl RequestMiddleware for AuthMiddleware {
    async fn handle(&self, req: &mut Request) -> Result<(), AppError> {
        // Authentication logic
        let token = req.headers().get("Authorization");
        validate_token(token)?;
        Ok(())
    }
}
```

## Dependencies

```
hyperlane-app ├── hyperlane_config  ← Workspace dependency
├── hyperlane_plugin  ← Workspace dependency
├── serde             ← Workspace dependency
├── tracing           ← Workspace dependency
├── hyperlane         ← Workspace dependency
└── hyperlane-utils   ← Workspace dependency
```

## License

This project is licensed under the MIT License. For more details, please see the [license](license) file.

## Contributing

Contributions are welcome! Please submit an issue or create a pull request.

## Contact

If you have any questions, please contact the author: [root@ltpp.vip](mailto:root@ltpp.vip).
