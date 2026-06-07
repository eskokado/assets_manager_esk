# Delta: web-admin-shell

## Requirements

### Requirement: Enhanced header and navigation toggle

The admin shell SHALL include a toggle button in the top bar to expand/collapse the sidebar. The top bar SHALL display the authenticated user's name and email on the right side. The sidebar SHALL include icons for each navigation item and a "Sair" (Logout) button at the bottom.

#### Scenario: Sidebar toggle expands/collapses

- **WHEN** the sidebar is expanded and the toggle button is clicked
- **THEN** THE sidebar collapses to a narrow version showing only icons
- **AND WHEN** the toggle button is clicked again
- **THEN** the sidebar expands to its full width

#### Scenario: User info in top bar

- **WHEN** an authenticated user views the admin shell
- **THEN** their name and email are visible in the right side of the header

#### Scenario: Logout button in sidebar

- **WHEN** the user clicks the "Sair" button in the sidebar
- **THEN** the application clears the session and redirects to `/login`
