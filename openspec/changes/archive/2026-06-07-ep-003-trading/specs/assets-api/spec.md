## MODIFIED Requirements

### Requirement: Update asset endpoint (Admin)

The API SHALL expose `PUT /api/assets/:id` requiring a valid Bearer JWT with role `Admin`, allowing update of name, asset type, currency, and active status (deactivation). Deactivation SHALL be rejected when the asset has open positions with quantity greater than zero.

#### Scenario: Admin updates asset

- **WHEN** a client with Admin role sends valid update fields to `PUT /api/assets/:id` for an existing asset
- **THEN** the response status is 200 and the body reflects the updated asset

#### Scenario: Admin deactivates asset without open positions

- **WHEN** a client with Admin role sets active to false on `PUT /api/assets/:id` for an asset with no open positions
- **THEN** the asset is persisted as inactive and future buy operations SHALL reject this asset

#### Scenario: Admin cannot deactivate asset with open positions

- **WHEN** a client with Admin role sets active to false on `PUT /api/assets/:id` for an asset with at least one position quantity greater than zero
- **THEN** the response status indicates a business validation error and the asset remains active

#### Scenario: Investor cannot update asset

- **WHEN** a client with Investor role sends `PUT /api/assets/:id`
- **THEN** the response status is 403
