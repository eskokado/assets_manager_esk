## Why

Com autenticação (EP-001) concluída, a plataforma **Assets Manage** ainda não possui o catálogo mestre de instrumentos financeiros. O épico EP-002 entrega o bounded context **Assets** — CRUD Admin de ativos, listagem autenticada e regras de ticker único e status ativo/inativo — desbloqueando EP-003 (Trading), que depende de ativos válidos para compra e venda.

## What Changes

- Criar módulo `assets` em `crates/api/src/modules/assets/` com domínio (Asset AR, VOs Ticker/AssetName/AssetType/Currency, AssetCatalogPolicy), application (CreateAsset, UpdateAsset, FindAssetById, ListAssets) e infra (AssetSqlxRepository)
- Adicionar migration `assets` (id, ticker unique, name, asset_type, currency, active, timestamps)
- Expor `GET /api/assets`, `GET /api/assets/:id`, `POST /api/assets`, `PUT /api/assets/:id` com autorização Admin para escrita e JWT para leitura
- Enforçar ticker único (RN-004), bloqueio de compra em ativo inativo (RN-005) e policy de desativação com posição aberta `[inferido]`
- Implementar telas Leptos admin `/admin/assets`, `/admin/assets/new`, `/admin/assets/:id/edit` com guard Admin
- Integrar menu "Ativos" no shell web visível apenas para role Admin
- Testes unitários (domain + application ≥95%), E2E admin cria/lista, investor GET lista e POST 403, testes use cases UI e CI verde

## Capabilities

### New Capabilities

- `assets-api`: Endpoints REST CRUD de catálogo; listagem paginada com filtros; autorização Admin para POST/PUT; leitura autenticada para Investidor
- `assets-persistence`: Schema `assets`, adapter sqlx AssetRepository com unique(ticker)
- `assets-web`: Telas Leptos admin de listagem e formulários create/edit, entidades UI, use cases e AssetHttpRepository

### Modified Capabilities

- `web-admin-shell`: Item de menu "Ativos" para Admin; guard `/admin/*` com 403 ou redirect para não-admin

## Impact

- **Código**: `crates/api/src/modules/assets/**`, migration em `migrations/`, `crates/web-leptos` (rotas admin assets, guards, forms)
- **APIs**: novos endpoints `/api/assets/*`; reutiliza middleware JWT e `AuthUser` role de EP-001
- **Dependências**: nenhuma crate nova significativa; Money/Currency pode reutilizar shared-kernel
- **Downstream**: EP-003 Trading consultará Assets para validar ativo ativo em BUY; EP-004 enriquece posições com ticker/nome
- **Documentação**: alinhado a `docs/planning/assets-manage/backlog.md` EP-002, US-003/US-004 e `ddd-tactical-model.md` BC Assets
