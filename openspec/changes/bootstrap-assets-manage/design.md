## Context

Greenfield **Assets Manage** — carteira de investimentos com backend Rust (Axum + sqlx) e frontend Leptos SSR. O backlog EP-000 exige monólito modular, Postgres, CI desde o início e shell web para demos. Não há specs OpenSpec prévias; esta change é a fundação do repositório.

**Referências**: `docs/planning/assets-manage/backlog.md` (US-000), `delivery-profile.md`, `config-project-rs`, `config-project-leptos`.

## Goals / Non-Goals

**Goals:**

- Workspace Cargo compilável com `shared-kernel`, `api` e `web-leptos`
- Postgres acessível em dev via docker-compose; migrations sqlx aplicáveis
- `GET /health` retornando 200
- Shared kernel com testes unitários e cobertura ≥95%
- Shell Leptos com sidebar/topbar/dashboard vazio
- CI GitHub Actions: fmt, clippy, test, coverage gate
- Dockerfiles multi-stage para api e web (produção)

**Non-Goals:**

- Auth, JWT, CRUD de ativos ou trading (épicos EP-001+)
- Mobile
- Integração com mercado/cotações
- OAuth ou RBAC granular

## Decisions

### 1. Monólito modular Cargo (não microserviços)

**Decisão**: Um workspace com crates `shared-kernel`, `api`, `web-leptos`.

**Rationale**: Equipe pequena, deploy único, transações ACID futuras entre Trading/Portfolio. Alinhado a `ddd-operational-notes.md`.

**Alternativa rejeitada**: Microserviços — complexidade prematura.

### 2. sqlx + migrations SQL (não ORM)

**Decisão**: sqlx com queries compile-time; migrations em `migrations/`.

**Rationale**: Stack padrão `config-sqlx-rs`; performance e controle explícito do schema.

### 3. Leptos SSR para web admin

**Decisão**: `crates/web-leptos` com cargo-leptos, Tailwind v4, proxy para API.

**Rationale**: Perfil de entrega R1; skills `config-project-leptos` + `config-shared-web-leptos`.

**Alternativa rejeitada**: SPA Vue/Angular — perfil fixou Leptos no delivery-profile.

### 4. Layout de módulos Axum

**Decisão**: `crates/api/src/modules/{health,...}` — um diretório por BC futuro.

**Rationale**: `config-new-module-rs` e namespace `modules::<bc>::domain::<Type>`.

### 5. CI Rust

**Decisão**: GitHub Actions com rustfmt, clippy, cargo test, coverage shared-kernel; job placeholder `memory-check` para EP-003+.

**Rationale**: `config-cicd-rs` e Epic DoD do repositório de skills.

## Risks / Trade-offs

| Risco | Mitigação |
|-------|-----------|
| sqlx compile-time exige DB em build CI | Service container Postgres no workflow ou `SQLX_OFFLINE=true` após cache |
| Leptos + Axum duas crates aumentam setup | EP-000 dedicado; docker-compose orquestra ambos |
| Cobertura 95% só no shared-kernel inicialmente | Gate CI restrito a shared-kernel até EP-001 expandir |

## Migration Plan

Greenfield — não há migração de legado. Deploy:

1. `docker compose up -d` (Postgres)
2. Aplicar migrations
3. `cargo run -p api` + `cargo leptos watch` (dev)
4. Produção via imagens Docker e compose prod (pós-CI verde)

**Rollback**: Reverter commit/imagens; sem dados de produção no bootstrap.

## Open Questions

- Seed de usuário Admin na migration vs. script separado → deferir para EP-001 Auth
- JWT secret naming (`JWT_SECRET`) → definir em `.env.example` no bootstrap, uso em EP-001
