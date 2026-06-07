## ADDED Requirements

### Requirement: Sell form asset pre-selection from portfolio

The Leptos web app route `/trades/sell` SHALL accept an optional query parameter `assetId`. When present and the user holds a position for that asset, the sell form SHALL pre-select that asset and display the available maxQuantity without requiring manual re-selection.

#### Scenario: Pre-select asset from query param

- **WHEN** an authenticated user navigates to `/trades/sell?assetId=:assetId` for an asset they hold
- **THEN** the sell form pre-selects that asset and shows available quantity

#### Scenario: Invalid assetId in query param

- **WHEN** an authenticated user navigates to `/trades/sell?assetId=:assetId` for an asset they do not hold
- **THEN** the form displays an error or clears selection with guidance to choose a held asset

#### Scenario: Sell form without query param unchanged

- **WHEN** an authenticated user navigates to `/trades/sell` without query parameters
- **THEN** the sell form behaves as before, requiring manual asset selection
