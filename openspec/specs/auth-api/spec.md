# auth-api Specification

## Purpose
TBD - created by archiving change ep-001-auth. Update Purpose after archive.
## Requirements
### Requirement: User registration endpoint

The API SHALL expose `POST /api/auth/register` accepting name, email, and password, creating a new user with role `Investor` and returning HTTP 201 with user profile data excluding password hash.

#### Scenario: Successful registration

- **WHEN** a client sends valid name, unique email, and password meeting minimum policy to `POST /api/auth/register`
- **THEN** the response status is 201 and the body contains user id, name, email, and role `Investor`

#### Scenario: Duplicate email rejected

- **WHEN** a client sends an email already registered to `POST /api/auth/register`
- **THEN** the response status indicates a business validation error and no duplicate user is created

### Requirement: User login endpoint

The API SHALL expose `POST /api/auth/login` accepting email and password, validating credentials and returning a JWT access token with claims including userId and role.

#### Scenario: Successful login

- **WHEN** a client sends valid registered credentials to `POST /api/auth/login`
- **THEN** the response status is 200 and the body contains a JWT access token and token metadata

#### Scenario: Invalid credentials rejected

- **WHEN** a client sends incorrect email or password to `POST /api/auth/login`
- **THEN** the response status is 401 and no token is returned

### Requirement: Authenticated profile endpoint

The API SHALL expose `GET /api/auth/me` requiring a valid Bearer JWT and returning the authenticated user's profile without password hash.

#### Scenario: Profile with valid token

- **WHEN** a client sends `GET /api/auth/me` with a valid Bearer JWT
- **THEN** the response status is 200 and the body contains id, name, email, and role

#### Scenario: Profile without token rejected

- **WHEN** a client sends `GET /api/auth/me` without a valid Bearer JWT
- **THEN** the response status is 401

### Requirement: JWT authentication middleware

The API SHALL provide reusable JWT validation middleware that extracts userId and role from Bearer tokens for protected routes.

#### Scenario: Valid token accepted by middleware

- **WHEN** a protected route receives a request with a valid Bearer JWT
- **THEN** the handler receives authenticated user context and processes the request

#### Scenario: Invalid or missing token rejected

- **WHEN** a protected route receives a request without a valid Bearer JWT
- **THEN** the response status is 401

