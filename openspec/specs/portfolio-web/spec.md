# portfolio-web Specification

## Purpose
TBD - created by archiving change ep-004-portfolio. Update Purpose after archive.
## Requirements
### Requirement: Portfolio list page

The Leptos web app SHALL expose route `/portfolio` for authenticated users, displaying the user's open positions with ticker, name, quantity, average price, and line total, plus aggregate totalInvested, or an empty state when no positions exist.

#### Scenario: Investor views portfolio

- **WHEN** an authenticated user navigates to `/portfolio`
- **THEN** the page displays positions fetched from GET `/api/portfolio`

#### Scenario: Unauthenticated user redirected

- **WHEN** an unauthenticated user navigates to `/portfolio`
- **THEN** the app redirects to `/login`

#### Scenario: Empty portfolio state

- **WHEN** an authenticated user with no open positions navigates to `/portfolio`
- **THEN** the page displays an empty state message with guidance to buy assets

#### Scenario: Navigate to position detail

- **WHEN** an authenticated user clicks a position row or link on `/portfolio`
- **THEN** the app navigates to `/portfolio/assets/:assetId` for that asset

### Requirement: Position detail page

The Leptos web app SHALL expose route `/portfolio/assets/:assetId` for authenticated users, displaying position detail including ticker, quantity, average price, totalInvested, and a control to initiate a sell for that asset.

#### Scenario: Position detail loads

- **WHEN** an authenticated user navigates to `/portfolio/assets/:assetId` for a held asset
- **THEN** the page displays position detail from GET `/api/portfolio/assets/:assetId`

#### Scenario: Position not found

- **WHEN** an authenticated user navigates to `/portfolio/assets/:assetId` for an asset they do not hold
- **THEN** the page displays a not-found or error state with link back to `/portfolio`

#### Scenario: Sell shortcut from detail

- **WHEN** an authenticated user clicks "Vender" on the position detail page
- **THEN** the app navigates to `/trades/sell?assetId=:assetId`

### Requirement: Portfolio presentation use cases

The web crate SHALL implement LoadPortfolioUseCase and LoadPositionUseCase orchestrating IPortfolioRepository with shared_kernel::Result return types.

#### Scenario: Load portfolio use case

- **WHEN** LoadPortfolioUseCase executes for authenticated session
- **THEN** it returns PortfolioSummary with positions and totalInvested from the portfolio HTTP repository

#### Scenario: Load position use case

- **WHEN** LoadPositionUseCase executes with a valid assetId
- **THEN** it returns PositionDetail for that asset or a not-found error

### Requirement: Portfolio HTTP repository

The PortfolioHttpRepository SHALL call GET `/api/portfolio` and GET `/api/portfolio/assets/:assetId` with Bearer JWT, mapping API DTOs to PortfolioSummary and PositionDetail domain entities.

#### Scenario: Repository sends JWT on list

- **WHEN** LoadPortfolioUseCase invokes the repository
- **THEN** the HTTP request includes Authorization Bearer header from session

#### Scenario: Repository maps detail response

- **WHEN** GET `/api/portfolio/assets/:assetId` returns position detail
- **THEN** the repository maps fields to PositionDetail including ticker, quantity, averagePrice, and totalInvested
