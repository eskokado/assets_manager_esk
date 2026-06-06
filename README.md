# assets_manage

Carteira de investimentos — Rust (Axum + sqlx) + Leptos SSR.

## Stack

- **API**: `crates/api` (Axum, PostgreSQL/sqlx)
- **Shared kernel**: `crates/shared-kernel`
- **Web**: `crates/web-leptos` (Leptos SSR + Tailwind v4)

## Quick start

```bash
cp .env.example .env
docker compose up -d
cargo build
cargo test -p shared-kernel
cargo run -p api
```

Health check: `GET http://localhost:4000/health`

## Leptos (dev)

```bash
npm install
npm run watch:css   # terminal 1
cargo leptos watch  # terminal 2 (requires cargo-leptos)
```

## Docs

- Discovery: `docs/discovery/assets-manage/`
- Planning: `docs/planning/assets-manage/`
- OpenSpec: `openspec/changes/bootstrap-assets-manage/`
