# trading-persistence Specification

## Purpose
TBD - created by archiving change ep-003-trading. Update Purpose after archive.
## Requirements
### Requirement: Trades table schema

The system SHALL persist trades in a `trades` table with columns: id (UUID PK), user_id (FK to users), asset_id (FK to assets), side (BUY or SELL), quantity (numeric > 0), unit_price (numeric > 0), currency (ISO 4217), traded_at (timestamptz), created_at (timestamptz). Trade records SHALL NOT be updated or deleted after insert.

#### Scenario: Trade record created on buy

- **WHEN** a BuyAsset use case completes successfully
- **THEN** a row exists in `trades` with side BUY and the submitted quantity and unit_price

#### Scenario: Trade record created on sell

- **WHEN** a SellAsset use case completes successfully
- **THEN** a row exists in `trades` with side SELL and the submitted quantity and unit_price

### Requirement: Positions table schema

The system SHALL persist positions in a `positions` table with composite primary key (user_id, asset_id), columns quantity (numeric ≥ 0), average_price (numeric ≥ 0), currency (ISO 4217), updated_at (timestamptz).

#### Scenario: Position created on first buy

- **WHEN** a user buys an asset for the first time
- **THEN** a position row exists with quantity equal to bought quantity and average_price equal to buy unit price

#### Scenario: Position updated on subsequent buy

- **WHEN** a user buys additional quantity of an asset they already hold
- **THEN** the position quantity increases and average_price is recalculated as weighted average

#### Scenario: Position reduced on sell

- **WHEN** a user sells partial quantity
- **THEN** the position quantity decreases by sold amount and average_price remains unchanged

#### Scenario: Position removed or zeroed on full sell

- **WHEN** a user sells entire position quantity
- **THEN** the position quantity becomes zero or the row is removed per implementation choice documented in design

### Requirement: Shared transaction for trade and position

The sqlx adapters for trades and positions SHALL support executing save trade and upsert position within the same sqlx transaction handle.

#### Scenario: Both adapters use same transaction

- **WHEN** BuyAsset or SellAsset executes
- **THEN** trade insert and position upsert share one transaction and commit or rollback together

### Requirement: Trade repository adapter

The TradeSqlxRepository SHALL implement TradeRepository port with save, findById, findByUserId (paginated with filters), and findByUserAndAsset operations mapping between TradeRecord and domain Trade entity.

#### Scenario: Paginated list by user

- **WHEN** ListTrades query requests page 1 with limit 20 for a user
- **THEN** the repository returns up to 20 trades ordered by traded_at descending for that user only

### Requirement: Portfolio repository adapter for writes

The PositionSqlxRepository SHALL implement PortfolioRepository port with findByUserAndAsset and upsert position operations for use by trading use cases.

#### Scenario: Find position for sell validation

- **WHEN** SellValidator queries position for user and asset
- **THEN** the repository returns current quantity and average_price or empty when no position exists
