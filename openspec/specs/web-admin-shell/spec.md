# web-admin-shell Specification

## Purpose
TBD - created by archiving change bootstrap-assets-manage. Update Purpose after archive.
## Requirements
### Requirement: Admin shell layout

The Leptos web crate SHALL render an admin shell with collapsible sidebar, top bar, footer, and an empty dashboard route. Public routes (`/login`, `/register`) SHALL render outside or without requiring the private shell guard. Authenticated users SHALL see a "Perfil" menu item linking to `/profile`. Private routes SHALL redirect unauthenticated users to `/login`.

#### Scenario: Dashboard renders shell

- **WHEN** an authenticated user navigates to the dashboard route in the running Leptos app
- **THEN** sidebar, topbar, and footer are visible with placeholder content

#### Scenario: API proxy configured

- **WHEN** the Leptos dev server runs locally
- **THEN** HTTP requests to the backend API are proxied or configured to the local API base URL

#### Scenario: Profile menu visible when authenticated

- **WHEN** an authenticated user views the admin shell
- **THEN** a menu item "Perfil" linking to `/profile` is visible

#### Scenario: Private route redirects to login

- **WHEN** an unauthenticated user navigates to a private route such as the dashboard
- **THEN** the app redirects to `/login`

