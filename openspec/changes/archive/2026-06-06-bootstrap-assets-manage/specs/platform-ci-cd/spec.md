## ADDED Requirements

### Requirement: Production Docker images

The project SHALL provide multi-stage Dockerfiles to build production images for the API and Leptos web crates, plus a `docker-compose.prod.yml` (or equivalent) for orchestration.

#### Scenario: Production compose builds

- **WHEN** a developer runs the documented production compose build command
- **THEN** Docker images for API and web are built successfully

### Requirement: Continuous integration pipeline

The project SHALL include a GitHub Actions workflow that on pull requests runs `cargo fmt --check`, `clippy`, `cargo test`, and enforces shared-kernel coverage ≥95%.

#### Scenario: CI passes on bootstrap PR

- **WHEN** a pull request is opened for the bootstrap change
- **THEN** the CI workflow completes successfully with all required jobs green
