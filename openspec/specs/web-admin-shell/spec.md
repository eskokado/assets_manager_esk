# web-admin-shell Specification

## Purpose
TBD - created by archiving change bootstrap-assets-manage. Update Purpose after archive.
## Requirements
### Requirement: Admin shell layout

The Leptos web crate SHALL render an admin shell with collapsible sidebar, top bar, footer, and an empty dashboard route. Public routes (`/login`, `/register`) SHALL render outside or without requiring the private shell guard. Authenticated users SHALL see a "Perfil" menu item linking to `/profile`. Users with Admin role SHALL see an "Ativos" menu item linking to `/admin/assets`. Authenticated users (Investor and Admin) SHALL see an "Operações" menu item linking to `/trades` with access to buy and sell flows at `/trades/buy` and `/trades/sell`. Routes under `/admin/*` SHALL require Admin role and deny or redirect non-Admin users. Private routes SHALL redirect unauthenticated users to `/login`.

#### Scenario: Dashboard renders shell

- **WHEN** an authenticated user navigates to the dashboard route in the running Leptos app
- **THEN** sidebar, topbar, and footer are visible with placeholder content

#### Scenario: API proxy configured

- **WHEN** the Leptos dev server runs locally
- **THEN** HTTP requests to the backend API are proxied or configured to the local API base URL

#### Scenario: Profile menu visible when authenticated

- **WHEN** an authenticated user views the admin shell
- **THEN** a menu item "Perfil" linking to `/profile` is visible

#### Scenario: Assets menu visible for Admin

- **WHEN** an authenticated user with Admin role views the admin shell
- **THEN** a menu item "Ativos" linking to `/admin/assets` is visible

#### Scenario: Assets menu hidden for Investor

- **WHEN** an authenticated user with Investor role views the admin shell
- **THEN** the "Ativos" menu item is not visible

#### Scenario: Operations menu visible when authenticated

- **WHEN** an authenticated user (Investor or Admin) views the admin shell
- **THEN** a menu item "Operações" linking to `/trades` is visible

#### Scenario: Operations menu hidden when unauthenticated

- **WHEN** an unauthenticated user views public pages
- **THEN** the "Operações" menu item is not visible

#### Scenario: Private route redirects to login

- **WHEN** an unauthenticated user navigates to a private route such as the dashboard
- **THEN** the app redirects to `/login`

#### Scenario: Admin route blocked for non-Admin

- **WHEN** an authenticated Investor navigates to `/admin/assets`
- **THEN** the app denies access with 403 messaging or redirects away from the admin area

### Requirement: Trading routes in private shell

Routes `/trades`, `/trades/buy`, and `/trades/sell` SHALL require authentication and render within the private shell layout.

#### Scenario: Trading route requires auth

- **WHEN** an unauthenticated user navigates to `/trades/buy`
- **THEN** the app redirects to `/login`

#### Scenario: Authenticated user accesses buy route

- **WHEN** an authenticated Investor navigates to `/trades/buy`
- **THEN** the buy form renders within the shell without admin guard

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
