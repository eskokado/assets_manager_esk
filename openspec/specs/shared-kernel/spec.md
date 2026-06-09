# shared-kernel Specification

## Purpose
TBD - created by archiving change bootstrap-assets-manage. Update Purpose after archive.
## Requirements
### Requirement: Shared domain primitives

The `shared-kernel` crate SHALL expose base types: `Entity`, `ValueObject`, `Result`, `UseCase` trait, and value objects `Id`, `Email`, and `Money`.

#### Scenario: Value object validation

- **WHEN** invalid email or negative money amount is passed to factory methods
- **THEN** the factory returns `Result::Err` with validation errors

#### Scenario: Unit test coverage

- **WHEN** unit tests run for `shared-kernel` with coverage enabled
- **THEN** line coverage for the crate is at least 95%

