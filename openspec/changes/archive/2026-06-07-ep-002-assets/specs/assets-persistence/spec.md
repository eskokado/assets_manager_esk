## ADDED Requirements

### Requirement: Assets database schema

The system SHALL provide a PostgreSQL table `assets` with columns id (UUID PK), ticker (unique, not null), name (not null), asset_type (not null), currency (not null), active (boolean, default true), created_at, and updated_at.

#### Scenario: Migration creates assets table

- **WHEN** database migrations are applied on a fresh Postgres instance
- **THEN** the `assets` table exists with unique constraint on ticker

#### Scenario: Duplicate ticker prevented at database level

- **WHEN** an insert attempts to persist a ticker that already exists
- **THEN** the database rejects the insert due to unique constraint on ticker

### Requirement: Asset sqlx repository adapter

The system SHALL implement an AssetRepository adapter using sqlx that maps between database records and the Asset domain aggregate, supporting save, find_by_id, find_by_ticker, find_all (paginated with filters), and update operations.

#### Scenario: Save new asset

- **WHEN** the repository save operation is called with a valid new Asset aggregate
- **THEN** a row is inserted in `assets` and the aggregate is returned with persisted timestamps

#### Scenario: Find by ticker

- **WHEN** the repository find_by_ticker operation is called with an existing ticker
- **THEN** the corresponding Asset aggregate is returned

#### Scenario: Paginated find all with filters

- **WHEN** the repository find_all operation is called with pagination and filter parameters
- **THEN** a page of Asset aggregates matching the filters is returned with total count metadata
