## Context

**Assets Manage** possui workspace Rust (EP-000) e autenticação JWT com roles Admin/Investor (EP-001). Não existe catálogo de instrumentos — Trading (EP-003) não pode validar ativos para compra/venda. O backlog EP-002 define o bounded context `assets` com CRUD Admin, listagem autenticada, ticker único e flag `active` que bloqueia novas compras.

**Referências**: `docs/planning/assets-manage/backlog.md` (US-003, US-004), `ddd-tactical-model.md` BC Assets, `delivery-profile.md` (Leptos admin, sem mobile).

## Goals / Non-Goals

**Goals:**

- Módulo `assets` completo inside-out: VOs → Asset AR → use cases/queries → sqlx → rotas HTTP
- Admin cria/edita/desativa ativos; Investidor lista e consulta detalhe (leitura)
- Ticker uppercase único; ativo inativo rejeitado em compras futuras (validação síncrona no EP-003)
- Telas admin `/admin/assets`, `/admin/assets/new`, `/admin/assets/:id/edit` com guard Admin
- Cobertura ≥95% domain+application do módulo assets; E2E admin CRUD + investor 403 em POST
- CI verde incluindo assets backend + Leptos admin

**Non-Goals:**

- Cotação de mercado, importação em massa, metadados por tipo além do enum
- Rota de catálogo para investidor fora do fluxo Trading (investidor usa listagem via API em EP-003)
- Mobile
- Eventos assíncronos AssetDeactivated — validação síncrona no MVP

## Decisions

### 1. Módulo Axum `modules/assets`

**Decisão**: Bounded context `modules/assets` com camadas domain/application/infrastructure/interfaces, seguindo padrão de `modules/auth`.

**Rationale**: Consistência com EP-001; skill `config-new-module-rs`.

**Alternativa rejeitada**: Crate separado `assets-core` — sem reuso cross-app no MVP.

### 2. VOs e Money do shared-kernel

**Decisão**: `Ticker`, `AssetName`, `AssetType` (enum STOCK|FII|ETF|BOND), `Currency` no domain assets; reutilizar `Money` ou padrão ISO 4217 do kernel para currency quando aplicável.

**Rationale**: RN-004/006; Currency default BRL conforme modelagem tática.

### 3. Autorização Admin via AuthUser extractor

**Decisão**: POST/PUT `/api/assets*` exigem `role === Admin`; GET exige JWT autenticado (Admin ou Investor).

**Rationale**: RF-005/008; reutiliza middleware JWT de EP-001.

**Alternativa rejeitada**: Permissões granulares — fora do escopo MVP.

### 4. AssetCatalogPolicy para desativação

**Decisão**: Domain service consulta posições abertas antes de desativar `[inferido]`; no MVP pode stub retornando OK se positions ainda não existirem (EP-003), com interface pronta para integração.

**Rationale**: RF-008 e ddd-tactical-model AssetCatalogPolicy.

### 5. Listagem paginada com filtros

**Decisão**: `ListAssets` query com page/limit, filtros `asset_type`, `active`, busca por ticker/name (ILIKE).

**Rationale**: RF-006 e critérios US-003.

### 6. Estrutura web Leptos admin

**Decisão**: `AssetListItem`, `AssetForm` + `IAssetRepository` (reqwest) + List/Load/Save use cases; páginas SSR com Resource para listagem e signals nos forms.

**Rationale**: Skills `frontend-entity-leptos` → `frontend-page-leptos`; rotas em `/admin/assets/*`.

### 7. Guard `/admin/*` no shell

**Decisão**: Middleware/guard Leptos verifica role Admin; Investidor recebe 403 ou redirect com mensagem de acesso negado.

**Rationale**: US-004 critérios; menu "Ativos" só para Admin.

## Risks / Trade-offs

| Risco | Mitigação |
|-------|-----------|
| AssetCatalogPolicy sem positions até EP-003 | Interface + stub; teste unitário com mock; integrar posição real em EP-003 |
| Ticker case sensitivity | Normalizar uppercase no VO Ticker::create |
| Duplicação de listagem ativos entre admin e trading | Investidor consome GET /api/assets no dropdown de EP-003; sem rota investidor dedicada no EP-002 |
| Cobertura 95% atrasa entrega | Gate CI expande para módulo assets; mocks de AssetRepository |

## Migration Plan

1. Aplicar migration `assets` em Postgres (dev/CI)
2. Deploy API com rotas `/api/assets/*` protegidas por JWT
3. Deploy web-leptos com rotas admin e menu "Ativos"
4. Seed opcional de ativos demo para dev (PETR4, HGLG11) via migration ou script

**Rollback**: Reverter migration assets se sem trades downstream; remover rotas admin do shell.

## Open Questions

- Seed de ativos demo: migration fixa vs script dev — preferir 2–3 ativos em seed SQL documentado
- AssetCatalogPolicy com positions: implementar consulta real quando EP-003 criar tabela positions, ou deferir bloqueio de desativação até lá
