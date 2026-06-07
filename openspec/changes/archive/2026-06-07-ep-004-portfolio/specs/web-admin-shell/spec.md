## ADDED Requirements

### Requirement: Portfolio menu in admin shell

Authenticated users (Investor and Admin) SHALL see a "Carteira" menu item linking to `/portfolio`. Routes `/portfolio` and `/portfolio/assets/:assetId` SHALL require authentication and render within the private shell layout.

#### Scenario: Portfolio menu visible when authenticated

- **WHEN** an authenticated user (Investor or Admin) views the admin shell
- **THEN** a menu item "Carteira" linking to `/portfolio` is visible

#### Scenario: Portfolio menu hidden when unauthenticated

- **WHEN** an unauthenticated user views public pages
- **THEN** the "Carteira" menu item is not visible

#### Scenario: Portfolio route requires auth

- **WHEN** an unauthenticated user navigates to `/portfolio`
- **THEN** the app redirects to `/login`

#### Scenario: Position detail route requires auth

- **WHEN** an unauthenticated user navigates to `/portfolio/assets/:assetId`
- **THEN** the app redirects to `/login`

#### Scenario: Authenticated user accesses portfolio route

- **WHEN** an authenticated Investor navigates to `/portfolio`
- **THEN** the portfolio page renders within the shell without admin guard
