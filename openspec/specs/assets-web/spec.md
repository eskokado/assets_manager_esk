# assets-web Specification

## Purpose
TBD - created by archiving change ep-002-assets. Update Purpose after archive.
## Requirements
### Requirement: Admin assets list page

The Leptos web app SHALL provide an `/admin/assets` route accessible only to users with Admin role, displaying a paginated table of assets with ticker, name, asset type, and active status loaded from `GET /api/assets`.

#### Scenario: Admin views asset list

- **WHEN** an Admin user navigates to `/admin/assets`
- **THEN** a table displays assets with ticker, name, type, and status columns

#### Scenario: Investor blocked from admin list

- **WHEN** an Investor user navigates to `/admin/assets`
- **THEN** the app shows access denied or redirects away from the admin area

### Requirement: Admin create asset page

The Leptos web app SHALL provide an `/admin/assets/new` route with a form that creates an asset via `POST /api/assets` and redirects to the list on success.

#### Scenario: Admin creates asset via form

- **WHEN** an Admin submits a valid asset form on `/admin/assets/new`
- **THEN** the asset is created and the user is redirected to `/admin/assets` where the new asset appears

#### Scenario: Duplicate ticker error on form

- **WHEN** an Admin submits a ticker that already exists on `/admin/assets/new`
- **THEN** an inline error is shown on the ticker field

### Requirement: Admin edit asset page

The Leptos web app SHALL provide an `/admin/assets/:id/edit` route with a form loaded from `GET /api/assets/:id` that updates the asset via `PUT /api/assets/:id`, including a toggle to deactivate the asset.

#### Scenario: Admin edits asset

- **WHEN** an Admin loads `/admin/assets/:id/edit` and submits valid changes
- **THEN** the asset is updated and the list reflects the changes

#### Scenario: Admin deactivates asset via form

- **WHEN** an Admin toggles active to false and saves on `/admin/assets/:id/edit`
- **THEN** the asset is persisted as inactive

### Requirement: Assets presentation layer

The web crate SHALL implement AssetListItem and AssetForm entities, IAssetRepository (HTTP adapter with Admin JWT), and ListAssetsUseCase, LoadAssetUseCase, and SaveAssetUseCase orchestrating the repository.

#### Scenario: List use case returns assets

- **WHEN** ListAssetsUseCase executes via IAssetRepository with valid Admin session
- **THEN** it returns Success with a list of AssetListItem entries

#### Scenario: Save use case handles validation errors

- **WHEN** SaveAssetUseCase executes with a duplicate ticker response from the API
- **THEN** it returns Failure with a message suitable for inline form display
