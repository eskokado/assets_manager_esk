# Perfil de entrega — Assets Manage

**Data**: 2026-06-06
**Legado**: Greenfield (descrição funcional — sem sistema legado)
**Combinação**: **R1** — Rust Axum + Leptos SSR (custom; ver `config-project-rs` + `config-project-leptos`)
**Inventário**: `docs/discovery/assets-manage/delivery-inventory.md`
**Modelagem**: `docs/modeling/assets-manage/ddd-tactical-model.md`

## Stack alvo (fixa)

| Camada | Escolha | Tecnologia | Pasta no monorepo |
|--------|---------|------------|-------------------|
| Backend | **RS** | Axum + Tokio + sqlx | `crates/api` |
| Shared kernel | **RS** | Entity, Result, UseCase | `crates/shared-kernel` |
| Web | **Leptos** | Leptos SSR + Tailwind CSS v4 | `crates/web-leptos` |
| Mobile | **Nenhum** | — | — |
| Banco | — | PostgreSQL + sqlx migrations | `migrations/` |

**Sufixo de skills**: `-rs` (backend) · Leptos skills sem sufixo (`frontend-*-leptos`)

**Detalhe web**: Leptos SSR (cargo-leptos), shell admin com sidebar/topbar (`Config Shared Web (Leptos)`), signals + Resource para listagens.

**Detalhe mobile**: N/A no MVP.

## Superfícies por Bounded Context (MVP — Release 1)

| BC / Épico | API REST | Web | Mobile | Observação |
|------------|----------|-----|--------|------------|
| EP-000 Bootstrap | Sim | Sim | Não | health, shell Leptos, docker, CI |
| EP-001 Auth | Sim | Sim | Não | login, register, profile |
| EP-002 Assets | Sim | Sim | Não | admin CRUD `/admin/assets/*` |
| EP-003 Trading | Sim | Sim | Não | buy, sell, histórico |
| EP-004 Portfolio | Sim | Sim | Não | carteira, detalhe posição |

## Resumo do inventário de UI

- **Web (inferido)**: painel investidor (carteira, operações) + área admin (catálogo de ativos) + auth (login/register)
- **Mobile**: não previsto
- **Somente API**: health check, integrações futuras (cotações, OAuth)

## Regras para o `backlog.md`

1. EP-000: `Config Project Full-Stack` → `Config Project (Rust)` + `Config Project (Leptos)` + docker + cicd + sqlx + shared kernel.
2. Cada US com Web=Sim inclui tasks `Frontend Entity (Leptos)` → … → `Frontend Page/Form (Leptos)`.
3. Épico Rust fecha com `test:unit`, `test:e2e`, `quality:ci-verify`, `quality:memory-leak`.
4. Auth MVP manual (sem `Config Auth Core` Rust) — domain + JWT no módulo `auth`.
