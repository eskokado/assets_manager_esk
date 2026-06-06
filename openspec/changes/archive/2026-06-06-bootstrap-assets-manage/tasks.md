# Tasks — bootstrap-assets-manage (EP-000)

**Story**: US-000 Setup do Projeto  
**Stack**: Rust (Axum + sqlx) · Leptos SSR · Postgres  
**Ref**: `docs/planning/assets-manage/backlog.md` EP-000

## 1. Orquestração e workspace

- [x] 1.1 `infra:fullstack` Orquestrar bootstrap Rust + Leptos (~1h)
  - **Agent:** `Config Project Full-Stack`
  - **Prompt:** Bootstrap Assets Manage: backend Rust Axum (config-project-rs), frontend Leptos SSR (config-project-leptos), Postgres sqlx, docker e cicd no EP-000. OpenSpec: bootstrap-assets-manage.

- [x] 1.2 `infra:setup` Bootstrap workspace Cargo (~2h)
  - **Agent:** `Config Project (Rust)`
  - **Prompt:** Inicialize workspace com crates/shared-kernel e crates/api (Axum). docker-compose Postgres, .env.example, health module.

- [x] 1.3 `infra:setup` Bootstrap frontend Leptos (~2h)
  - **Agent:** `Config Project (Leptos)`
  - **Prompt:** Crie crate web-leptos com cargo-leptos, Tailwind v4, proxy para API local.

## 2. Shared kernel

- [x] 2.1 `domain:shared` Shared kernel (~2h)
  - **Agent:** `Config Shared Core (Rust)`
  - **Prompt:** Crie Entity, ValueObject, Result, UseCase, Id, Email, Money em shared-kernel com testes unitários.

- [x] 2.2 `test:unit` Testes shared-kernel (~1h)
  - **Agent:** `Unit Tests (Rust)`
  - **Prompt:** Garanta testes Id, Email, Money, Result com cobertura ≥95% em shared-kernel.

## 3. Database infra

- [x] 3.1 `infra:db` Configurar sqlx + migrations (~1h)
  - **Agent:** `Config SQLx (Rust)`
  - **Prompt:** Configure DATABASE_URL, pool sqlx, pasta migrations/, migrate no startup ou script.

## 4. API — health

- [x] 4.1 `interface:controller` Health check (~30min)
  - **Agent:** `Backend Controller (Rust)`
  - **Prompt:** GET /health em modules/health retornando status ok.

- [x] 4.2 `test:e2e` E2E health (~1h)
  - **Agent:** `E2E Tests (Rust)`
  - **Prompt:** Integration test: sobe app, GET /health → 200.

## 5. Web — admin shell

- [x] 5.1 `infra:shell-web` Shell admin Tailwind (~1h)
  - **Agent:** `Config Shared Web (Leptos)`
  - **Prompt:** Configure shell: sidebar colapsável, topbar, rodapé, dashboard vazio e rotas de referência.

## 6. Docker e CI/CD

- [x] 6.1 `infra:docker` Docker multi-stage produção (~1h)
  - **Agent:** `Config Docker (Rust)`
  - **Prompt:** Dockerfile multi-stage para api e web-leptos; docker-compose.prod.yml.

- [x] 6.2 `infra:cicd` Pipeline GitHub Actions (~2h)
  - **Agent:** `Config CI/CD (Rust)`
  - **Prompt:** CI: fmt, clippy, cargo test, coverage shared-kernel ≥95%. Job memory-check placeholder.

## 7. Fechamento EP-000

- [x] 7.1 `quality:ci-verify` Pipeline verde (~30min)
  - **Agent:** `Config CI/CD (Rust)`
  - **Prompt:** Abrir PR de bootstrap; corrigir falhas até CI verde.

## Critérios de aceitação (US-000)

- [x] `cargo build` e `cargo test -p shared-kernel` passam
- [x] `GET /health` retorna 200
- [x] Postgres sobe via docker-compose; migrations aplicam
- [x] Pipeline CI executa clippy, test, coverage shared-kernel
- [x] Shell Leptos exibe layout com sidebar, topbar e dashboard vazio
