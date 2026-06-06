## Why

O projeto **Assets Manage** (carteira de investimentos em Rust) ainda não possui código — apenas discovery, modelagem DDD e backlog. É necessário um bootstrap técnico (EP-000) que estabeleça workspace Cargo, Postgres, CI/CD, API mínima e shell web Leptos antes de implementar os bounded contexts Auth, Assets, Trading e Portfolio.

## What Changes

- Criar monorepo Rust: `crates/shared-kernel`, `crates/api` (Axum), `crates/web-leptos` (SSR)
- Configurar PostgreSQL via docker-compose, sqlx e pasta `migrations/`
- Implementar shared kernel (Entity, Result, UseCase, Id, Email, Money) com testes
- Expor `GET /health` na API
- Configurar shell admin Leptos (sidebar, topbar, dashboard vazio)
- Adicionar Docker multi-stage de produção e pipeline GitHub Actions (fmt, clippy, test, coverage)
- Preparar estrutura modular em `crates/api/src/modules/` para épicos seguintes

## Capabilities

### New Capabilities

- `platform-bootstrap`: Workspace Cargo, docker-compose dev, `.env.example`, módulo health scaffold
- `shared-kernel`: Tipos base de domínio compartilhados (Entity, Result, UseCase, VOs Id/Email/Money)
- `database-infra`: Pool sqlx, migrations versionadas, integração com API
- `health-api`: Endpoint HTTP de saúde da API
- `web-admin-shell`: Shell Leptos SSR com layout admin e dashboard placeholder
- `platform-ci-cd`: Dockerfile produção, docker-compose.prod, GitHub Actions Rust

### Modified Capabilities

- _(nenhuma — greenfield)_

## Impact

- **Código novo**: raiz do repositório (`Cargo.toml`, `crates/*`, `migrations/`, `.github/workflows/`)
- **Infra**: docker-compose, variáveis `DATABASE_URL`, secrets CI
- **Dependências**: Axum, sqlx, tokio, leptos, cargo-leptos
- **Documentação**: alinhado a `docs/planning/assets-manage/delivery-profile.md` (stack Rust + Leptos)
- **Épicos desbloqueados**: EP-001 Auth, EP-002 Assets, EP-003 Trading, EP-004 Portfolio
