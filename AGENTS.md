# Agent Development Guide

## Build & Test Commands
- Build: `cargo build`
- Run tests: `cargo test`
- Run single test: `cargo test <test_name>` (e.g., `cargo test return_success_when_user_exists`)
- Check: `cargo check`
- Format: `cargo fmt` (if available)
- Lint: `cargo clippy` (if available)

## Code Style & Conventions
- **Imports**: Group std, external crates, then local crates; use `use crate::` for local imports
- **Error Handling**: Use `anyhow::Result` for repositories, `AppResult<T>` (alias for `Result<T, Error>`) for services; return custom `Error` enum variants; use `.context()` for error messages
- **Async**: Use `#[async_trait]` for async trait methods
- **Types**: Strongly typed with explicit types; use type aliases like `DynUsersRepository = Arc<dyn UsersRepository + Send + Sync>`
- **Naming**: snake_case for functions/variables, PascalCase for types/traits, suffix traits with `Trait` for service traits
- **Logging**: Use `tracing::{info, error, debug}` macros; log at entry/exit points and errors
- **Testing**: Use mockall for mocking; structure tests with arrange/act/assert comments; name tests descriptively (e.g., `return_error_when_user_exists`)
- **Architecture**: Repository pattern for DB ops; services contain business logic; controllers handle HTTP layer; DTOs for data transfer

## Project Structure
- `src/database/<table>/`: model.rs (schema), repository.rs (DB ops), mod.rs
- `src/server/`: api/ (controllers), dtos/, services/ (business logic), utils/, error.rs
- `migrations/`: SQL migration files
- `tests/`: Integration/unit tests
