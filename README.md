# assets_manage

Carteira de investimentos — Rust (Axum + sqlx) + Leptos SSR.

## Stack

- **API**: `crates/api` (Axum, PostgreSQL/sqlx)
- **Shared kernel**: `crates/shared-kernel`
- **Web**: `crates/web-leptos` (Leptos SSR + Tailwind v4)

## Ambientes

| | Desenvolvimento | Produção |
|---|-----------------|----------|
| **Compose** | `docker-compose.yml` | `docker-compose.prod.yml` |
| **O que sobe no Docker** | Somente **Postgres** | **Postgres** + **API** |
| **API** | Host — `cargo run -p api` (`:4000`) | Container — imagem do `Dockerfile` |
| **Web (Leptos)** | Host — `cargo leptos watch` (`:3000`) | *(web em prod: ver nota abaixo)* |
| **Hot reload** | Sim (cargo / leptos watch) | Não — rebuild de imagem |

**Por que dev no host?** Compilação incremental, `cargo leptos watch` e sqlx apontando para `localhost:5432` — padrão definido no EP-000 e mantido nos épicos de negócio (EP-001+).

> **Nota prod:** o `docker-compose.prod.yml` atual sobe API + Postgres. Imagem do frontend Leptos pode ser adicionada quando fechar o deploy full-stack.

## Desenvolvimento (recomendado)

Pré-requisitos: Rust stable, Docker, Node.js (Tailwind), [cargo-leptos](https://github.com/leptos-rs/cargo-leptos).

```bash
cp .env.example .env
docker compose up -d          # 1) só Postgres em :5432
```

**Terminal 1 — API**

```bash
cargo run -p api              # :4000 — migrations aplicam no startup
```

**Terminal 2 — CSS (opcional se já buildou)**

```bash
npm install
npm run watch:css
```

**Terminal 3 — Web**

```bash
cargo leptos watch            # :3000 — proxy para API_BASE_URL
```

### Verificação rápida

| URL | Esperado |
|-----|----------|
| `GET http://localhost:4000/health` | `{"status":"ok"}` |
| `http://localhost:3000/login` | Tela de login |
| `http://localhost:3000/` | Dashboard (exige login) |

### Auth (EP-001)

- **API:** `POST /api/auth/register`, `POST /api/auth/login`, `GET /api/auth/me` (Bearer JWT)
- **Web:** `/login`, `/register`, `/profile`
- **Seed dev:** `admin@assets.local` / `Admin1234` (migration `users`)

### Testes

```bash
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

Requer Postgres acessível (`DATABASE_URL` no `.env`).

## Produção

Build da API e stack com Docker Compose de produção:

```bash
docker compose -f docker-compose.prod.yml up -d --build
```

Variáveis: ajuste `DATABASE_URL`, `JWT_SECRET` e `BIND_ADDR` no compose ou via env antes do deploy.

## Docs

- Discovery: `docs/discovery/assets-manage/`
- Planning: `docs/planning/assets-manage/`
- OpenSpec (ativo): `openspec/changes/`
- OpenSpec (arquivado): `openspec/changes/archive/`
