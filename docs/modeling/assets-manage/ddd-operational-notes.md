# Notas Operacionais — Assets Manage

**Baseado em**: modelo estratégico e tático (2026-06-06)
**Stack alvo**: Rust — Axum, sqlx, PostgreSQL, shared-kernel

## Topologia Recomendada

**Monólito modular** (recomendado para MVP)

| Critério | Avaliação | Decisão |
| -------- | --------- | ------- |
| Equipe | Projeto prático / pequena equipe | Monólito |
| Deploy | Cadência única | Monólito |
| Escala | Carga uniforme, sem picos extremos no MVP | Monólito |
| Maturidade | Domínio em exploração (greenfield) | Monólito |
| Consistência | Buy/Sell + Portfolio exige transação ACID | Monólito facilita |

```
┌────────────────────────────────────────────────────────────┐
│              MONÓLITO MODULAR — crates/api                  │
├────────────────────────────────────────────────────────────┤
│  modules/auth │ modules/assets │ modules/portfolio │ trading │
│       └──────────────┴─────────────────┴──────────────┘     │
│                    PostgreSQL (sqlx)                        │
└────────────────────────────────────────────────────────────┘
```

**Evolução futura**: extrair BC Trading ou Portfolio para serviço separado somente se houver evidência de escala/deploy independente — não antecipar no MVP.

---

## Mapeamento BC → Módulo → Skills

| BC | Módulo Cargo | Scaffold | Domain | Application | Infrastructure | Interface |
| -- | ------------ | -------- | ------ | ----------- | -------------- | --------- |
| Auth | `modules/auth` | `config-new-module-rs` | `core-entity-rs`, `core-value-object-rs`, `core-repository-rs` | `core-use-case-rs`, `core-dto-rs` | `backend-data-rs`, `config-sqlx-rs` | `backend-controller-rs` |
| Assets | `modules/assets` | idem | idem | idem + `core-query-cqrs-rs` | idem | idem |
| Portfolio | `modules/portfolio` | idem | idem + `core-domain-service-rs` | idem + queries | idem | idem |
| Trading | `modules/trading` | idem | idem + `core-domain-service-rs` | idem | idem | idem |
| Shared | `crates/shared-kernel` | `config-shared-core-rs` | Entity, Result, UseCase, Id, Money, Email | — | — | — |

### Bootstrap e infra transversal

| Épico | Skills |
| ----- | ------ |
| EP-000 Bootstrap | `config-project-rs`, `config-sqlx-rs`, `config-docker-rs`, `config-cicd-rs` |
| Testes | `test-unit-rs`, `test-e2e-rs` |
| Frontend (opcional) | `config-project-leptos`, `config-shared-web-leptos`, `frontend-*-leptos` |

---

## Prioridade de Implementação

Ordem baseada em dependências do context map e subdomínios Core:

| Ordem | BC / Épico | Justificativa |
| ----- | ---------- | ------------- |
| 1 | **EP-000 Bootstrap** | Workspace Cargo, Postgres, CI, health check |
| 2 | **Auth** | Upstream — desbloqueia guards e identidade em todos os BCs |
| 3 | **Assets** | Catálogo necessário antes de negociar instrumentos |
| 4 | **Trading** | Core — compra/venda é fluxo principal de valor |
| 5 | **Portfolio** | Core — projeção de posições (pode ser implementado junto com Trading na mesma transação) |
| 6 | **Frontend web** `[inferido]` | Demo visual após API estável |

> **Nota**: Trading e Portfolio são fortemente acoplados na mesma transação DB; no backlog, considerar **um épico compartilhado** ou stories encadeadas na mesma sprint.

---

## Decisões técnicas transversais

| Tópico | Decisão MVP |
| ------ | ----------- |
| Persistência | PostgreSQL + sqlx (compile-time checked queries) |
| Transação | `BuyAsset` / `SellAsset` envolvem `Trade` + `Portfolio` na mesma TX |
| Auth | JWT stateless; bcrypt para hash de senha |
| Portfolio | Materializado (tabela `positions`) — recalculado a cada trade |
| Eventos | Sem message broker no MVP; integração Trading→Portfolio síncrona |
| Migrations | SQL versionado em `migrations/` |
| Moeda padrão | BRL `[inferido]` |

---

## Superfícies de entrega (resumo)

| BC | API REST | Web | Mobile |
| -- | -------- | --- | ------ |
| Auth | Sim | Sim (login/register/profile) | Não |
| Assets | Sim | Sim (admin CRUD) | Não |
| Portfolio | Sim | Sim (painel investidor) | Não |
| Trading | Sim | Sim (compra/venda/histórico) | Não |

Frontend web é **opcional** para fechar MVP de backend; recomendado para demonstração (`delivery-profile.md` no `req-agile-planning`).
