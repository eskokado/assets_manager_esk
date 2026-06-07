# portfolio-api Specification

## Purpose
TBD - created by archiving change ep-004-portfolio. Update Purpose after archive.
## Requirements
### Requirement: Get user portfolio

The API SHALL expose `GET /api/portfolio` for authenticated users, returning the JWT owner's portfolio with a list of positions where quantity is greater than zero, each enriched with asset ticker and name, plus a computed totalInvested sum (sum of quantity × average_price per position).

#### Scenario: Portfolio with open positions

- **WHEN** an authenticated user with open positions requests GET `/api/portfolio`
- **THEN** the response status is 200 and includes positions with assetId, ticker, name, quantity, averagePrice, currency, totalInvested per line, and aggregate totalInvested

#### Scenario: Empty portfolio

- **WHEN** an authenticated user with no open positions requests GET `/api/portfolio`
- **THEN** the response status is 200 with an empty positions array and totalInvested zero

#### Scenario: Unauthenticated request rejected

- **WHEN** a request without valid JWT calls GET `/api/portfolio`
- **THEN** the response status is 401

#### Scenario: Owner isolation

- **WHEN** user A requests GET `/api/portfolio`
- **THEN** only positions belonging to user A are returned

### Requirement: Get position by asset

The API SHALL expose `GET /api/portfolio/assets/:assetId` for authenticated users, returning the JWT owner's position detail for the given asset when quantity is greater than zero.

#### Scenario: Existing position detail

- **WHEN** an authenticated user requests GET `/api/portfolio/assets/:assetId` for an asset they hold with quantity greater than zero
- **THEN** the response status is 200 with assetId, ticker, name, quantity, averagePrice, currency, and totalInvested

#### Scenario: Position not found

- **WHEN** an authenticated user requests GET `/api/portfolio/assets/:assetId` for an asset they do not hold or with zero quantity
- **THEN** the response status is 404

#### Scenario: Unauthenticated position detail rejected

- **WHEN** a request without valid JWT calls GET `/api/portfolio/assets/:assetId`
- **THEN** the response status is 401

### Requirement: Portfolio query authorization

Portfolio endpoints SHALL derive userId exclusively from the JWT token and SHALL NOT accept userId as a query or path parameter for cross-user access.

#### Scenario: No cross-user portfolio access

- **WHEN** user A is authenticated and requests portfolio endpoints
- **THEN** the system never returns positions belonging to user B

### Requirement: Portfolio read model from positions table

Portfolio queries SHALL read from the existing `positions` table materialized by trading use cases, joining `assets` for ticker and name enrichment, without performing writes or recalculating average price.

#### Scenario: Portfolio reflects last trade

- **WHEN** a buy trade completes successfully for user U and asset A
- **THEN** a subsequent GET `/api/portfolio` for user U includes the updated quantity and averagePrice for asset A
