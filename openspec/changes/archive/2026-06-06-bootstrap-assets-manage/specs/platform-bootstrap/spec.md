## ADDED Requirements

### Requirement: Cargo workspace structure

The project SHALL provide a Rust workspace at the repository root with at least the crates `shared-kernel`, `api`, and `web-leptos`.

#### Scenario: Workspace builds successfully

- **WHEN** a developer runs `cargo build` at the repository root
- **THEN** all workspace members compile without errors

#### Scenario: Development environment starts

- **WHEN** a developer runs `docker compose up -d` using the provided compose file
- **THEN** PostgreSQL becomes reachable on the configured host and port

#### Scenario: Environment template exists

- **WHEN** a developer copies `.env.example` to `.env`
- **THEN** required variables for database and API ports are documented and sufficient for local development

### Requirement: API module layout

The `api` crate SHALL expose a `modules` directory prepared for bounded-context modules, including an initial `health` module.

#### Scenario: Health module registered

- **WHEN** the API binary starts
- **THEN** routes from `modules/health` are mounted on the Axum router
