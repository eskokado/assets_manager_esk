# Tasks — ep-003-trading (EP-003)

**Stories**: US-005 API de compra e venda · US-006 Telas de negociação (Leptos)  
**Stack**: Rust (Axum + sqlx) · Leptos SSR · Postgres  
**Ref**: `docs/planning/assets-manage/backlog.md` EP-003

## 1. Domínio — Value Objects, entidade, services e ports

- [x] 1.1 `domain:vo` TradeSide, Quantity, UnitPrice, TradeTotal (~2h)
  - **Agent:** `Core Value Object (Rust)`
  - **Prompt:** Crie VOs TradeSide (enum BUY|SELL), Quantity (>0 decimal), UnitPrice (Money >0), TradeTotal (quantity × unitPrice) em modules/trading/domain. Create() retorna shared_kernel::Result.

- [x] 1.2 `domain:entity` Trade aggregate (~2h)
  - **Agent:** `Core Entity (Rust)`
  - **Prompt:** Aggregate Trade imutável após create. Atributos: id, userId, assetId, side, quantity, unitPrice, currency, tradedAt, createdAt. Factory create() valida VOs. modules/trading/domain/entity.

- [x] 1.3 `domain:service` TradeExecutor, SellValidator (~2h)
  - **Agent:** `Core Domain Service (Rust)`
  - **Prompt:** TradeExecutor valida ativo ativo via AssetRepository (BUY), orquestra persist trade + portfolio update. SellValidator garante quantity solicitada ≤ posição atual via PortfolioRepository.

- [x] 1.4 `domain:service` PortfolioCalculator (~2h)
  - **Agent:** `Core Domain Service (Rust)`
  - **Prompt:** PortfolioCalculator recalcula quantity e average_price após BUY (média ponderada); reduz quantity após SELL mantendo average_price. PositionValidator garante quantity nunca negativa.

- [x] 1.5 `domain:repository` TradeRepository, PortfolioRepository ports (~1h)
  - **Agent:** `Core Repository (Rust)`
  - **Prompt:** Traits TradeRepository (save, findById, findByUserId paginated, findByUserAndAsset) e PortfolioRepository (findByUserAndAsset, upsert position). Sem update/delete em trades. modules/trading/domain/ports.

## 2. Application — DTOs, use cases e queries

- [x] 2.1 `app:dto` BuyIn, SellIn, TradeOut (~1h)
  - **Agent:** `Core DTO (Rust)`
  - **Prompt:** DTOs serde BuyIn (assetId, quantity, unitPrice, tradedAt?), SellIn (idem), TradeOut (id, side, assetId, ticker?, quantity, unitPrice, total, tradedAt). modules/trading/application/dto.

- [x] 2.2 `app:usecase` BuyAsset, SellAsset (~3h)
  - **Agent:** `Core Use Case (Rust)`
  - **Prompt:** BuyAsset e SellAsset em sqlx transaction: validar → save trade → upsert position via PortfolioCalculator. Rollback em falha parcial. Reutilizar TransactionManager/pool pattern de EP-001.

- [x] 2.3 `app:query` ListTrades, FindTradeById (~2h)
  - **Agent:** `Core Query CQRS (Rust)`
  - **Prompt:** ListTrades paginada por userId JWT owner; filtros side, assetId, from/to. FindTradeById retorna TradeOut ou NotFound se não owner. modules/trading/application/query.

## 3. Infraestrutura — migrations e persistência

- [x] 3.1 `infra:migration` trades + positions (~1h)
  - **Agent:** `Config SQLx (Rust)`
  - **Prompt:** Tabela trades: id UUID PK, user_id FK, asset_id FK, side, quantity, unit_price, currency, traded_at, created_at. Tabela positions: PK (user_id, asset_id), quantity, average_price, currency, updated_at. migrations/.

- [x] 3.2 `infra:persistence` Trade + Position adapters (~3h)
  - **Agent:** `Backend Data (Rust)`
  - **Prompt:** TradeSqlxRepository e PositionSqlxRepository; mapeamento TradeRecord/PositionRecord ↔ domain. Suporte a TX compartilhada (mesmo &mut Transaction). Upsert position.

- [x] 3.3 `infra:integration` AssetCatalogPolicy com positions (~1h)
  - **Agent:** `Core Domain Service (Rust)`
  - **Prompt:** Atualizar AssetCatalogPolicy em modules/assets para consultar positions antes de desativar; rejeitar se qty > 0. Substituir stub de EP-002.

## 4. API — rotas REST

- [x] 4.1 `interface:controller` Rotas /api/trades/* (~2h)
  - **Agent:** `Backend Controller (Rust)`
  - **Prompt:** POST /api/trades/buy, POST /api/trades/sell, GET /api/trades (paginated + filtros), GET /api/trades/:id. JWT owner; userId do token. Mapear Result para status 201/200/404/422.

## 5. Testes backend

- [x] 5.1 `test:unit` Trading + calculator (~3h)
  - **Agent:** `Unit Tests (Rust)`
  - **Prompt:** Testes SellValidator, PortfolioCalculator, TradeExecutor (mocks), BuyAsset, SellAsset com mocks de repos. Cobertura ≥95% domain+application do módulo trading.

- [x] 5.2 `test:e2e` Fluxo buy → sell (~2h)
  - **Agent:** `E2E Tests (Rust)`
  - **Prompt:** E2E: login investor, POST buy ativo ativo, verify position qty, GET trades lista BUY, sell parcial OK, sell excesso falha business error, buy ativo inativo falha.

## 6. Web — entidades, use cases e repository

- [x] 6.1 `interface:entity` TradeListItem, BuyTradeForm, SellTradeForm (~1h)
  - **Agent:** `Frontend Entity (Leptos)`
  - **Prompt:** Entidades UI TradeListItem (id, side, ticker, quantity, unitPrice, total, tradedAt), BuyTradeForm (assetId, quantity, unitPrice), SellTradeForm (assetId, quantity, unitPrice, maxQuantity). shared_kernel::Result, sem leptos::* no domain. crate web-leptos.

- [x] 6.2 `interface:usecase` List, Buy, Sell, LoadActiveAssets (~2h)
  - **Agent:** `Frontend UseCase (Leptos)`
  - **Prompt:** ListTradesUseCase, BuyAssetUseCase, SellAssetUseCase, LoadActiveAssetsUseCase async orquestrando ITradeRepository e IAssetRepository; retorno Result. LoadActiveAssets filtra active=true para dropdown /trades/buy.

- [x] 6.3 `interface:repository` TradeHttpRepository (~2h)
  - **Agent:** `Frontend Repository (Leptos)`
  - **Prompt:** reqwest adapter: POST /api/trades/buy, POST /api/trades/sell, GET /api/trades com JWT. Mapear DTOs API → TradeListItem/BuyTradeForm/SellTradeForm.

## 7. Web — páginas, formulários e shell

- [x] 7.1 `interface:page` Histórico trades (~2h)
  - **Agent:** `Frontend Page (Leptos)`
  - **Prompt:** Página /trades com Resource paginada; tabela side, ticker, quantity, unitPrice, total, tradedAt. Links para /trades/buy e /trades/sell. Empty state quando sem trades.

- [x] 7.2 `interface:form-web` Forms buy e sell (~2h)
  - **Agent:** `Frontend Form (Leptos)`
  - **Prompt:** Forms /trades/buy (dropdown ativos ativos, qty, preço) e /trades/sell (dropdown posições ou ativo+maxQuantity, qty, preço). Erro saldo insuficiente com qty disponível. Sucesso redirect /trades ou /portfolio.

- [x] 7.3 `infra:shell-web` Menu Operações no shell (~1h)
  - **Agent:** `Config Shared Web (Leptos)`
  - **Prompt:** Menu "Operações" visível para usuário autenticado (Investor e Admin), link /trades. Rotas /trades/* no shell privado; guard auth sem admin guard.

## 8. Testes web e fechamento

- [x] 8.1 `test:unit-web` Trading UI use cases (~1h)
  - **Agent:** `Frontend UseCase (Leptos)`
  - **Prompt:** Testes unitários BuyAssetUseCase, SellAssetUseCase, ListTradesUseCase com mock ITradeRepository. Cobrir erro saldo insuficiente e ativo inativo.

- [x] 8.2 `quality:ci-verify` + `quality:memory-leak` (~1h)
  - **Agent:** `Config CI/CD (Rust)`
  - **Prompt:** CI verde EP-003; clippy, test, coverage trading module; memory check após E2E trading.

## Critérios de aceitação (US-005)

- [x] Dado ativo ativo, quando BUY qty>0 e price>0, então trade criado e posição aumentada
- [x] Dado saldo insuficiente, quando SELL, então erro de negócio
- [x] Dado ativo inativo, quando BUY, então rejeitado
- [x] Dado usuário autenticado, quando GET trades, então histórico paginado próprio
- [x] Dado BUY e update portfolio, quando falha parcial, então rollback TX

## Critérios de aceitação (US-006)

- [x] Dado `/trades/buy`, quando seleciona ativo ativo e confirma, então operação registrada
- [x] Dado `/trades/sell`, quando qty > posição, então erro exibido com saldo disponível
- [x] Dado `/trades`, quando carrega, então lista BUY/SELL com datas e totais
