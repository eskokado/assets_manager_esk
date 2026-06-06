# Inventário de entrega — Assets Manage

**Fonte**: discovery 2026-06-06
**Legado**: Greenfield (sem sistema legado — superfícies inferidas para MVP)

## Resumo

| Superfície              | Presente no legado? | Observação                                      |
| ----------------------- | ------------------- | ----------------------------------------------- |
| Painel web admin        | Não (greenfield)    | `[inferido]` CRUD ativos + gestão usuários      |
| Painel web investidor   | Não (greenfield)    | `[inferido]` carteira, compra/venda, histórico  |
| App mobile              | Não                 | Fora do escopo MVP                              |
| API REST (Axum)         | Sim (alvo)          | Superfície principal do MVP Rust                |

## Telas web por área de negócio

| Área (≈ BC futuro) | Telas / rotas principais                              | CRUD? | Relatórios?        |
| ------------------ | ----------------------------------------------------- | ----- | ------------------ |
| Auth               | `/login`, `/register`, `/profile`                     | —     | —                  |
| Assets             | `/admin/assets`, `/admin/assets/new`, `/admin/assets/:id/edit` | Sim (Admin) | —      |
| Portfolio          | `/portfolio`, `/portfolio/assets/:id`                 | Read  | Resumo posições    |
| Trading            | `/trades`, `/trades/buy`, `/trades/sell`              | Create + Read | Histórico      |

> Frontend web não é obrigatório para fechar o MVP de backend; recomendado para demonstração do fluxo completo `[inferido]`. Stack frontend a definir em `delivery-profile.md` (Leptos SSR ou SPA separada).

## API por área (MVP obrigatório)

| Área     | Endpoints principais                          | Autenticação   |
| -------- | --------------------------------------------- | -------------- |
| Auth     | register, login, me                           | Público / JWT    |
| Assets   | CRUD `/api/assets`                            | Admin write    |
| Portfolio| GET `/api/portfolio`                          | JWT (owner)    |
| Trading  | POST buy/sell, GET trades                     | JWT (owner)    |

## Mobile (se aplicável)

| Fluxo    | Telas | Paridade MVP? |
| -------- | ----- | ------------- |
| —        | —     | Não previsto  |

## Próximo passo

→ `req-ddd-modeling` deve espelhar colunas **API | Web | Mobile** por BC.
→ `req-agile-planning` cria `planning/assets-manage/delivery-profile.md` com stack **Rust/Axum + Postgres** e frontend opcional antes do `backlog.md`.
