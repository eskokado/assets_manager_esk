# database-infra Specification

## Purpose
TBD - created by archiving change bootstrap-assets-manage. Update Purpose after archive.
## Requirements
### Requirement: SQLx database pool

The API SHALL configure a PostgreSQL connection pool using sqlx from `DATABASE_URL`.

#### Scenario: Pool connects on startup

- **WHEN** the API starts with a valid `DATABASE_URL` and Postgres is running
- **THEN** the application obtains a healthy connection pool without panic

### Requirement: Versioned migrations

The project SHALL maintain SQL migrations under `migrations/` and provide a documented way to apply them before or during API startup.

#### Scenario: Migrations apply cleanly

- **WHEN** migrations are executed against an empty database
- **THEN** the schema is created without errors

