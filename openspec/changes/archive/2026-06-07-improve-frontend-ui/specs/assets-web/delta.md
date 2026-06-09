# Delta: assets-web

## Requirements

### Requirement: Professional Assets Dashboard

The dashboard page SHALL present a professional view of the user's asset portfolio, including key metrics (KPIs) and recent activity.

#### Scenario: Dashboard KPI summary

- **WHEN** an authenticated user views the dashboard
- **THEN** they see cards showing "Saldo Total", "Rentabilidade", "Total de Ativos" and "Próximos Dividendos" (mocked if necessary)

#### Scenario: Recent transactions list

- **WHEN** an authenticated user views the dashboard
- **THEN** a list of at least the last 5 transactions is displayed with date, asset, type (buy/sell), and value
