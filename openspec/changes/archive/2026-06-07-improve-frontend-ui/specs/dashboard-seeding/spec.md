# dashboard-seeding Specification

## Purpose

The purpose of this capability is to provide realistic sample data for development and demonstration of the Assets Manager dashboard.

## Requirements

### Requirement: Database seeding script

The project SHALL include a script or mechanism to populate the database with realistic mock data for users, assets, categories, portfolios, and transactions.

#### Scenario: Seed execution

- **WHEN** the seeding command is executed
- **THEN** the database is populated with data that allows the dashboard to display non-empty states for all its components
