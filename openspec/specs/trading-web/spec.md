# trading-web Specification

## Purpose
TBD - created by archiving change ep-003-trading. Update Purpose after archive.
## Requirements
### Requirement: Trade history page

The Leptos web app SHALL expose route `/trades` for authenticated users, displaying a paginated list of BUY and SELL operations with side, ticker, quantity, unit price, total, and traded date.

#### Scenario: Investor views trade history

- **WHEN** an authenticated user navigates to `/trades`
- **THEN** the page displays a list of their trades fetched from GET `/api/trades`

#### Scenario: Unauthenticated user redirected

- **WHEN** an unauthenticated user navigates to `/trades`
- **THEN** the app redirects to `/login`

#### Scenario: Empty history state

- **WHEN** an authenticated user with no trades navigates to `/trades`
- **THEN** the page displays an empty state message

### Requirement: Buy trade form page

The Leptos web app SHALL expose route `/trades/buy` with a form to select an active asset, enter quantity and unit price, and submit a buy operation via POST `/api/trades/buy`.

#### Scenario: Successful buy submission

- **WHEN** an authenticated user selects an active asset, enters valid quantity and unit price, and submits the buy form
- **THEN** the operation is registered and the user receives success feedback with redirect to `/trades` or `/portfolio`

#### Scenario: Inactive asset not selectable

- **WHEN** the buy form loads asset options
- **THEN** only assets with active status true are available in the dropdown

#### Scenario: Buy validation errors displayed

- **WHEN** the API returns a validation or business error on buy
- **THEN** the form displays error messages without losing entered field values where applicable

### Requirement: Sell trade form page

The Leptos web app SHALL expose route `/trades/sell` with a form to select an asset the user holds, enter quantity and unit price, and submit a sell operation via POST `/api/trades/sell`, displaying available quantity as maxQuantity.

#### Scenario: Successful sell submission

- **WHEN** an authenticated user selects a held asset, enters quantity within available balance and valid unit price, and submits
- **THEN** the sell operation is registered and success feedback is shown

#### Scenario: Sell quantity exceeds balance

- **WHEN** the user enters quantity greater than available position
- **THEN** the form or API error displays a clear message with available quantity

#### Scenario: Sell form shows max quantity

- **WHEN** the user selects an asset on the sell form
- **THEN** the available quantity for that asset is visible to guide input

### Requirement: Trading presentation use cases

The web crate SHALL implement ListTradesUseCase, BuyAssetUseCase, SellAssetUseCase, and LoadActiveAssetsUseCase orchestrating ITradeRepository and IAssetRepository with shared_kernel::Result return types.

#### Scenario: List trades use case

- **WHEN** ListTradesUseCase executes for authenticated session
- **THEN** it returns paginated TradeListItem entities from the trade HTTP repository

#### Scenario: Load active assets for buy dropdown

- **WHEN** LoadActiveAssetsUseCase executes
- **THEN** it returns active assets suitable for the buy form asset selector

### Requirement: Trade HTTP repository

The TradeHttpRepository SHALL call POST `/api/trades/buy`, POST `/api/trades/sell`, and GET `/api/trades` with Bearer JWT, mapping API DTOs to TradeListItem, BuyTradeForm, and SellTradeForm domain entities.

#### Scenario: Repository sends JWT on buy

- **WHEN** BuyAssetUseCase invokes the repository
- **THEN** the HTTP request includes Authorization Bearer header from session

#### Scenario: Repository maps list response

- **WHEN** GET `/api/trades` returns paginated trades
- **THEN** the repository maps each item to TradeListItem with side, ticker, quantity, unitPrice, total, tradedAt
