## ADDED Requirements

### Requirement: Health endpoint

The API SHALL expose `GET /health` that returns HTTP 200 and a JSON body indicating the service is operational.

#### Scenario: Successful health check

- **WHEN** a client sends `GET /health` to the running API
- **THEN** the response status is 200 and the body indicates ok status

#### Scenario: E2E health verification

- **WHEN** the integration test suite runs the health E2E test
- **THEN** the test passes against a running API instance
