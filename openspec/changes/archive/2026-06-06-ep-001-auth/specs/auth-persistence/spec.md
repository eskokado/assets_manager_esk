## ADDED Requirements

### Requirement: Users table schema

The project SHALL maintain a `users` table via versioned SQL migration with columns: id (UUID or equivalent), name, email (unique), password_hash, role, active flag, and created_at timestamp.

#### Scenario: Migration creates users table

- **WHEN** migrations are applied against an empty database after bootstrap migrations
- **THEN** the `users` table exists with unique constraint on email

### Requirement: Password hash storage

The persistence layer SHALL store only bcrypt password hashes and SHALL never persist plaintext passwords.

#### Scenario: User saved with hash only

- **WHEN** a new user is persisted through UserRepository
- **THEN** the database row contains a bcrypt hash in password_hash and no plaintext password column

### Requirement: User repository adapter

The auth module SHALL provide a sqlx adapter implementing UserRepository with operations save, find_by_id, find_by_email, and exists_by_email.

#### Scenario: Find user by email

- **WHEN** find_by_email is called with a registered email
- **THEN** the adapter returns the corresponding User domain entity

#### Scenario: Check email uniqueness

- **WHEN** exists_by_email is called with a registered email
- **THEN** the adapter returns true
