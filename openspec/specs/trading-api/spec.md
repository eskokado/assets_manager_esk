# trading-api Specification

## Purpose
TBD - created by archiving change ep-003-trading. Update Purpose after archive.
## Requirements
### Requirement: Buy asset endpoint

The API SHALL expose `POST /api/trades/buy` requiring a valid Bearer JWT, accepting assetId, quantity, unitPrice, and optional tradedAt, creating an immutable BUY trade and updating the user's position in the same database transaction.

#### Scenario: Successful buy on active asset

- **WHEN** an authenticated user sends valid assetId, quantity > 0, and unitPrice > 0 for an active asset to `POST /api/trades/buy`
- **THEN** the response status is 201, the body contains the created trade with side BUY, and the user's position quantity is increased

#### Scenario: Buy rejected for inactive asset

- **WHEN** an authenticated user sends `POST /api/trades/buy` for an asset with active status false
- **THEN** the response status indicates a business validation error and no trade or position change is persisted

#### Scenario: Buy rejected for invalid quantity or price

- **WHEN** an authenticated user sends quantity ≤ 0 or unitPrice ≤ 0 to `POST /api/trades/buy`
- **THEN** the response status indicates a validation error and no trade is created

#### Scenario: Unauthenticated buy rejected

- **WHEN** a client sends `POST /api/trades/buy` without a valid Bearer JWT
- **THEN** the response status is 401

### Requirement: Sell asset endpoint

The API SHALL expose `POST /api/trades/sell` requiring a valid Bearer JWT, accepting assetId, quantity, unitPrice, and optional tradedAt, creating an immutable SELL trade and reducing the user's position in the same database transaction.

#### Scenario: Successful sell with sufficient balance

- **WHEN** an authenticated user sends valid sell payload with quantity less than or equal to current position to `POST /api/trades/sell`
- **THEN** the response status is 201, the body contains the created trade with side SELL, and the position quantity is reduced accordingly

#### Scenario: Sell rejected for insufficient balance

- **WHEN** an authenticated user sends sell quantity greater than current position quantity
- **THEN** the response status indicates a business validation error with insufficient balance semantics and no trade or position change is persisted

#### Scenario: Sell rejected for invalid quantity or price

- **WHEN** an authenticated user sends quantity ≤ 0 or unitPrice ≤ 0 to `POST /api/trades/sell`
- **THEN** the response status indicates a validation error and no trade is created

#### Scenario: Unauthenticated sell rejected

- **WHEN** a client sends `POST /api/trades/sell` without a valid Bearer JWT
- **THEN** the response status is 401

### Requirement: List trades endpoint

The API SHALL expose `GET /api/trades` requiring a valid Bearer JWT, returning a paginated list of the authenticated user's trades with optional filters for side, assetId, and date range.

#### Scenario: User lists own trades

- **WHEN** an authenticated user sends `GET /api/trades`
- **THEN** the response status is 200 and the body contains a paginated list of trades belonging only to that user

#### Scenario: Filter by side

- **WHEN** an authenticated user sends `GET /api/trades?side=BUY`
- **THEN** the response contains only trades with side BUY

#### Scenario: Filter by asset

- **WHEN** an authenticated user sends `GET /api/trades?assetId=<uuid>`
- **THEN** the response contains only trades for the specified asset

#### Scenario: Unauthenticated list rejected

- **WHEN** a client sends `GET /api/trades` without a valid Bearer JWT
- **THEN** the response status is 401

### Requirement: Get trade by id endpoint

The API SHALL expose `GET /api/trades/:id` requiring a valid Bearer JWT, returning trade detail when the trade belongs to the authenticated user, or HTTP 404 when not found or not owned.

#### Scenario: Owner retrieves trade

- **WHEN** an authenticated user sends `GET /api/trades/:id` for a trade they own
- **THEN** the response status is 200 and the body contains trade detail including side, quantity, unitPrice, total, and tradedAt

#### Scenario: Trade not found or not owned

- **WHEN** an authenticated user sends `GET /api/trades/:id` for a non-existent id or a trade owned by another user
- **THEN** the response status is 404

#### Scenario: Unauthenticated get rejected

- **WHEN** a client sends `GET /api/trades/:id` without a valid Bearer JWT
- **THEN** the response status is 401

### Requirement: Trade immutability contract

The API SHALL NOT expose update or delete endpoints for trades. Once persisted, a trade record SHALL remain immutable for audit purposes.

#### Scenario: No update endpoint

- **WHEN** a client sends PUT or PATCH to `/api/trades/:id`
- **THEN** the response status is 404 or 405 and no trade is modified

#### Scenario: No delete endpoint

- **WHEN** a client sends DELETE to `/api/trades/:id`
- **THEN** the response status is 404 or 405 and no trade is removed

### Requirement: Atomic trade and position update

Buy and sell operations SHALL persist the trade and update the corresponding position within a single database transaction such that partial failure results in full rollback.

#### Scenario: Rollback on position update failure

- **WHEN** a buy or sell operation fails during position upsert after trade insert within the same transaction
- **THEN** neither the trade nor the position change is committed
