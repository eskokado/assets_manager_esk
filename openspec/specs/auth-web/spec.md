# auth-web Specification

## Purpose
TBD - created by archiving change ep-001-auth. Update Purpose after archive.
## Requirements
### Requirement: Login page

The Leptos web app SHALL provide a `/login` route with a form that authenticates via `POST /api/auth/login` and redirects authenticated users to the dashboard or portfolio area.

#### Scenario: Successful login redirect

- **WHEN** a user submits valid credentials on `/login`
- **THEN** the session is stored and the user is redirected away from `/login`

#### Scenario: Login validation errors displayed

- **WHEN** a user submits invalid credentials on `/login`
- **THEN** inline error messages are shown without navigation

### Requirement: Registration page

The Leptos web app SHALL provide a `/register` route with a form that creates an account via `POST /api/auth/register` and redirects to login on success.

#### Scenario: Successful registration flow

- **WHEN** a user submits a valid registration form on `/register`
- **THEN** the account is created and the user is redirected to `/login`

### Requirement: Profile page

The Leptos web app SHALL provide a `/profile` route accessible only to authenticated users, displaying name, email, and role from `GET /api/auth/me`.

#### Scenario: Profile displays authenticated user

- **WHEN** an authenticated user navigates to `/profile`
- **THEN** name, email, and role are displayed

### Requirement: Route guard for private pages

The Leptos web app SHALL redirect unauthenticated users to `/login` when they attempt to access private routes including `/profile`.

#### Scenario: Unauthenticated access blocked

- **WHEN** a user without a valid session navigates to `/profile`
- **THEN** the app redirects to `/login`

### Requirement: Auth presentation layer

The web crate SHALL implement AuthSession and UserProfile entities, IAuthRepository (HTTP adapter), and LoginUseCase, RegisterUseCase, and LoadProfileUseCase orchestrating the repository.

#### Scenario: Login use case persists session

- **WHEN** LoginUseCase executes with valid credentials via IAuthRepository
- **THEN** it returns Success with AuthSession containing access token and role

