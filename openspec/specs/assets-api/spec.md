# assets-api Specification

## Purpose
TBD - created by archiving change ep-002-assets. Update Purpose after archive.
## Requirements
### Requirement: Create asset endpoint (Admin)

The API SHALL expose `POST /api/assets` requiring a valid Bearer JWT with role `Admin`, accepting ticker, name, asset type, currency, and active status, persisting a new asset with unique ticker and returning HTTP 201.

#### Scenario: Admin creates asset successfully

- **WHEN** a client with Admin role sends valid ticker, name, asset type, and currency to `POST /api/assets`
- **THEN** the response status is 201 and the body contains the created asset with id, ticker, name, asset type, currency, and active status

#### Scenario: Investor cannot create asset

- **WHEN** a client with Investor role sends a valid create payload to `POST /api/assets`
- **THEN** the response status is 403 and no asset is created

#### Scenario: Duplicate ticker rejected

- **WHEN** a client with Admin role sends a ticker that already exists to `POST /api/assets`
- **THEN** the response status indicates a business validation error and no duplicate asset is created

#### Scenario: Unauthenticated create rejected

- **WHEN** a client sends `POST /api/assets` without a valid Bearer JWT
- **THEN** the response status is 401

### Requirement: List assets endpoint

The API SHALL expose `GET /api/assets` requiring a valid Bearer JWT, returning a paginated list of assets with optional filters for asset type, active status, and search by ticker or name.

#### Scenario: Authenticated user lists assets

- **WHEN** a client with a valid Bearer JWT sends `GET /api/assets`
- **THEN** the response status is 200 and the body contains a paginated list of assets

#### Scenario: Filter by active status

- **WHEN** a client sends `GET /api/assets?active=true`
- **THEN** the response contains only assets with active status true

#### Scenario: Unauthenticated list rejected

- **WHEN** a client sends `GET /api/assets` without a valid Bearer JWT
- **THEN** the response status is 401

### Requirement: Get asset by id endpoint

The API SHALL expose `GET /api/assets/:id` requiring a valid Bearer JWT, returning asset detail or HTTP 404 when not found.

#### Scenario: Asset found

- **WHEN** a client with a valid Bearer JWT sends `GET /api/assets/:id` for an existing asset id
- **THEN** the response status is 200 and the body contains asset detail

#### Scenario: Asset not found

- **WHEN** a client sends `GET /api/assets/:id` for a non-existent id
- **THEN** the response status is 404

### Requirement: Update asset endpoint (Admin)

The API SHALL expose `PUT /api/assets/:id` requiring a valid Bearer JWT with role `Admin`, allowing update of name, asset type, currency, and active status (deactivation).

#### Scenario: Admin updates asset

- **WHEN** a client with Admin role sends valid update fields to `PUT /api/assets/:id` for an existing asset
- **THEN** the response status is 200 and the body reflects the updated asset

#### Scenario: Admin deactivates asset

- **WHEN** a client with Admin role sets active to false on `PUT /api/assets/:id`
- **THEN** the asset is persisted as inactive and future buy operations SHALL reject this asset

#### Scenario: Investor cannot update asset

- **WHEN** a client with Investor role sends `PUT /api/assets/:id`
- **THEN** the response status is 403

### Requirement: Asset active status enforcement contract

The API SHALL persist and expose the `active` flag on assets such that inactive assets remain readable but are marked unsuitable for new buy operations in downstream Trading context.

#### Scenario: Inactive asset readable

- **WHEN** a client requests `GET /api/assets/:id` for an inactive asset
- **THEN** the response status is 200 and active is false in the body
