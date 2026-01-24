# Gemini Project Information

This document provides an overview of the `moviebackend` project to guide development and ensure consistency.

## Project Overview

This project is a backend REST API written in Rust. It uses a PostgreSQL database for data storage.

## Directory Structure

- **`src/`**: Contains all the source code for the backend API.
    - **`database/`**: Manages the PostgreSQL database interactions.
        - **`<table_name>/`**: Each subdirectory represents a database table.
            - `model.rs`: Defines the table's schema/model.
            - `repository.rs`: Contains functions for database operations (CRUD, etc.) for the corresponding table.
            - `mod.rs`: Declares the module to the Rust compiler.
    - **`server/`**: Contains the web server and API logic.
        - **`api/`**: Defines the low-level API endpoints (controllers). These controllers are responsible for logging requests and calling the appropriate services.
        - **`dtos/`**: Contains Data Transfer Objects used for structuring data in API requests and responses.
        - **`extractors/`**: Custom extractors for Axum to handle things like session cookies, user agent parsing, and request validation.
        - **`services/`**: Contains the business logic for the API. These services are called by the controllers.
        - **`utils/`**: Contains utility functions, such as JWT creation/validation and password hashing.
        - `error.rs`: Defines custom error types and middleware for error handling.
- **`migrations/`**: Contains SQL files for database migrations, managed by `sqlx-cli`.
- **`tests/`**: Contains integration and unit tests.
- **`Cargo.toml`**: The Rust project manifest file, defining dependencies and project metadata.

## Development Workflow

When adding a new feature that involves a new database table, the typical workflow is:

1.  Create a new migration file in `migrations/`.
2.  Create a new subdirectory under `src/database/` for the new table.
3.  Create `model.rs` and `repository.rs` for the new table.
4.  Add a new DTO in `src/server/dtos/` if needed.
5.  Add a new controller in `src/server/api/` for the new endpoints.
6.  Add a new service in `src/server/services/` to implement the business logic.
7.  Add tests in the `tests/` directory.

## Service Implementation Notes

When implementing a service method, the following patterns are observed:

*   **Check for Existence:** Before creating a new entity, check if it already exists in the database to avoid duplicates. For example, in `waitlist_signup_user`, we first check if a user with the same email already exists.
*   **Repository Interaction:** The service layer relies on the repository layer for all database interactions. Ensure that the repository trait defines all necessary methods for the service's logic. For instance, the `waitlist_remove_user` service method requires a corresponding `delete_user` method in the `WaitlistUsersRepository` trait and its implementation.
