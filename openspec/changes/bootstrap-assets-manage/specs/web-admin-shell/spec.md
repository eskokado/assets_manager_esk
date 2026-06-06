## ADDED Requirements

### Requirement: Admin shell layout

The Leptos web crate SHALL render an admin shell with collapsible sidebar, top bar, footer, and an empty dashboard route.

#### Scenario: Dashboard renders shell

- **WHEN** a user navigates to the dashboard route in the running Leptos app
- **THEN** sidebar, topbar, and footer are visible with placeholder content

#### Scenario: API proxy configured

- **WHEN** the Leptos dev server runs locally
- **THEN** HTTP requests to the backend API are proxied or configured to the local API base URL
