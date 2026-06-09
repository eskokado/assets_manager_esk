# Tasks — ep-002-assets (EP-002)

**Stories**: US-003 API de catálogo de ativos · US-004 Telas admin de ativos (Leptos)  
**Stack**: Rust (Axum + sqlx) · Leptos SSR · Postgres  
**Ref**: `docs/planning/assets-manage/backlog.md` EP-002

## 1. Domínio — Value Objects, entidade, service e port

- [x] 1.1 `domain:vo` VOs Ticker, AssetName, AssetType, Currency (~2h)
  - **Agent:** `Core Value Object (Rust)`
  - **Prompt:** Crie VOs Ticker (uppercase unique, alfanumérico), AssetName, AssetType enum STOCK|FII|ETF|BOND, Currency ISO 4217 (default BRL) em modules/assets/domain. Create() retorna Result.

- [x] 1.2 `domain:entity` Asset aggregate (~2h)
  - **Agent:** `Core Entity (Rust)`
  - **Prompt:** Aggregate Asset com create() e deactivate(). Bloqueia compra se inactive. Atributos: id, ticker, name, asset_type, currency, active, created_at, updated_at. modules/assets/domain/entity.

- [x] 1.3 `domain:service` AssetCatalogPolicy (~1h)
  - **Agent:** `Core Domain Service (Rust)`
  - **Prompt:** AssetCatalogPolicy valida desativação; consulta posições abertas [inferido]. Interface pronta para integração EP-003; stub OK se positions ainda não existirem.

- [x] 1.4 `domain:repository` AssetRepository port (~1h)
  - **Agent:** `Core Repository (Rust)`
  - **Prompt:** Trait AssetRepository: save, find_by_id, find_by_ticker, find_all (paginated + filtros), update. modules/assets/domain/ports.

## 2. Application — DTOs, use cases e queries

- [x] 2.1 `app:dto` CreateAssetIn, UpdateAssetIn, AssetOut (~1h)
  - **Agent:** `Core DTO (Rust)`
  - **Prompt:** DTOs serde entrada/saída para asset. CreateAssetIn, UpdateAssetIn, AssetOut com paginação ListAssetsOut. modules/assets/application/dto.

- [x] 2.2 `app:usecase` CreateAsset, UpdateAsset (~2h)
  - **Agent:** `Core Use Case (Rust)`
  - **Prompt:** CreateAsset Admin-only; valida ticker único. UpdateAsset permite editar nome, tipo, currency e desativar via AssetCatalogPolicy.

- [x] 2.3 `app:query` FindAssetById, ListAssets (~2h)
  - **Agent:** `Core Query CQRS (Rust)`
  - **Prompt:** FindAssetById retorna AssetOut ou NotFound. ListAssets paginada com filtros asset_type, active, busca ticker/name (ILIKE).

## 3. Infraestrutura — migration e persistência

- [x] 3.1 `infra:migration` Migration assets (~1h)
  - **Agent:** `Config SQLx (Rust)`
  - **Prompt:** Tabela assets: id UUID PK, ticker unique not null, name, asset_type, currency, active default true, created_at, updated_at. Seed opcional PETR4, HGLG11 para dev.

- [x] 3.2 `infra:persistence` AssetSqlxRepository (~2h)
  - **Agent:** `Backend Data (Rust)`
  - **Prompt:** Adapter sqlx para assets; mapeamento AssetRecord ↔ Asset. Implementa AssetRepository com unique(ticker) e paginação.

## 4. API — rotas REST

- [x] 4.1 `interface:controller` Rotas /api/assets (~2h)
  - **Agent:** `Backend Controller (Rust)`
  - **Prompt:** GET /api/assets (JWT autenticado), GET /api/assets/:id, POST /api/assets (Admin), PUT /api/assets/:id (Admin). Reutiliza AuthUser extractor de EP-001. Investor POST/PUT → 403.

## 5. Testes backend

- [x] 5.1 `test:unit` Testes domain + use cases (~2h)
  - **Agent:** `Unit Tests (Rust)`
  - **Prompt:** Testes VOs, Asset, AssetCatalogPolicy, CreateAsset, UpdateAsset, FindAssetById, ListAssets com mocks. Cobertura ≥95% domain+application do módulo assets.

- [x] 5.2 `test:e2e` E2E catálogo (~2h)
  - **Agent:** `E2E Tests (Rust)`
  - **Prompt:** E2E: login Admin, POST /api/assets (201), GET /api/assets lista ativo. Login Investor, GET /api/assets (200), POST /api/assets (403). Ticker duplicado retorna erro de negócio.

## 6. Web — entidades, use cases e repository

- [x] 6.1 `interface:entity` AssetListItem, AssetForm (~1h)
  - **Agent:** `Frontend Entity (Leptos)`
  - **Prompt:** Entidades AssetListItem (id, ticker, name, assetType, active) e AssetForm (ticker, name, assetType, currency, active) com shared_kernel::Result, sem leptos::* no domain. crate web-leptos.

- [x] 6.2 `interface:usecase` List, Load, Save asset (~2h)
  - **Agent:** `Frontend UseCase (Leptos)`
  - **Prompt:** ListAssetsUseCase, LoadAssetUseCase, SaveAssetUseCase async orquestrando IAssetRepository; retorno Result. Save distingue create vs update.

- [x] 6.3 `interface:repository` AssetHttpRepository (~2h)
  - **Agent:** `Frontend Repository (Leptos)`
  - **Prompt:** reqwest adapter CRUD assets com JWT Admin: GET list, GET :id, POST, PUT. Mapear DTOs API → AssetListItem/AssetForm.

## 7. Web — formulários, páginas e shell

- [x] 7.1 `interface:page` Listagem admin (~2h)
  - **Agent:** `Frontend Page (Leptos)`
  - **Prompt:** Página /admin/assets com Resource e tabela (ticker, nome, tipo, status). Paginação e filtros. Link para /admin/assets/new e /admin/assets/:id/edit.

- [x] 7.2 `interface:form-web` Form create/edit (~2h)
  - **Agent:** `Frontend Form (Leptos)`
  - **Prompt:** Forms /admin/assets/new e /admin/assets/:id/edit com signals; submit chama SaveAssetUseCase; toggle active; erros de ticker duplicado inline no campo ticker.

- [x] 7.3 `infra:shell-web` Integrar assets no shell (~1h)
  - **Agent:** `Config Shared Web (Leptos)`
  - **Prompt:** Menu "Ativos" visível apenas se role Admin, link /admin/assets. Guard /admin/* exige Admin; Investor recebe 403 ou redirect com mensagem de acesso negado.

## 8. Testes web e fechamento

- [x] 8.1 `test:unit-web` Use cases asset UI (~1h)
  - **Agent:** `Frontend UseCase (Leptos)`
  - **Prompt:** Testes unitários ListAssetsUseCase, LoadAssetUseCase, SaveAssetUseCase com mock IAssetRepository. Cobrir erro ticker duplicado.

- [x] 8.2 `quality:ci-verify` CI verde EP-002 (~30min)
  - **Agent:** `Config CI/CD (Rust)`
  - **Prompt:** CI verde com módulo assets backend + telas Leptos admin + E2E catálogo.

## Critérios de aceitação (US-003)

- [x] Dado Admin autenticado, quando POST asset, então ativo criado com ticker único
- [x] Dado Investidor, quando POST asset, então retorna 403
- [x] Dado ativo inativo, quando investidor tenta compra (EP-003), então bloqueado
- [x] Dado filtros, quando GET assets, então lista paginada

## Critérios de aceitação (US-004)

- [x] Dado Admin em `/admin/assets`, quando lista carrega, então exibe ticker, nome, tipo, status
- [x] Dado Admin em `/admin/assets/new`, quando salva, então ativo aparece na listagem
- [x] Dado Investidor, quando acessa `/admin/assets`, então 403 ou redirect
