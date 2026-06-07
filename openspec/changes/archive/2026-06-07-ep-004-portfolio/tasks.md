# Tasks — ep-004-portfolio (EP-004)

**Stories**: US-007 API de carteira · US-008 Telas de carteira (Leptos)  
**Stack**: Rust (Axum + sqlx) · Leptos SSR · Postgres  
**Ref**: `docs/planning/assets-manage/backlog.md` EP-004

## 1. Domínio — aggregate, query port e reutilização de Position

- [x] 1.1 `domain:entity` Portfolio aggregate (~2h)
  - **Agent:** `Core Entity (Rust)`
  - **Prompt:** Crie aggregate Portfolio (AR) em modules/portfolio/domain com userId e coleção de Position filhas. Reutilize struct Position de modules/trading/domain (re-export ou dependência interna). Invariante: positions com quantity > 0 na listagem. Factory from_positions().

- [x] 1.2 `domain:repository` PortfolioQuery port read-only (~1h)
  - **Agent:** `Core Repository (Rust)`
  - **Prompt:** Trait PortfolioQuery com findByUserId(userId) -> Portfolio e findPosition(userId, assetId) -> Option<Position>. Sem métodos de write. modules/portfolio/domain/ports.

## 2. Application — DTOs e queries CQRS

- [x] 2.1 `app:dto` PortfolioOut, PositionOut (~1h)
  - **Agent:** `Core DTO (Rust)`
  - **Prompt:** DTOs serde PortfolioOut (totalInvested, positions[]), PositionOut (assetId, assetTicker, assetName, quantity, averagePrice, currency, totalInvested). modules/portfolio/application/dto.

- [x] 2.2 `app:query` GetUserPortfolio (~2h)
  - **Agent:** `Core Query CQRS (Rust)`
  - **Prompt:** Query GetUserPortfolio recebe userId do JWT owner; delega PortfolioQuery; mapeia para PortfolioOut com totalInvested agregado. Filtra positions quantity > 0.

- [x] 2.3 `app:query` GetPositionByAsset (~1h)
  - **Agent:** `Core Query CQRS (Rust)`
  - **Prompt:** Query GetPositionByAsset(userId, assetId) retorna PositionOut enriquecido ou NotFound se ausente ou qty zero. Owner-only.

## 3. Infraestrutura — adapter sqlx read-only

- [x] 3.1 `infra:persistence` PortfolioQuerySqlx (~2h)
  - **Agent:** `Backend Data (Rust)`
  - **Prompt:** Adapter sqlx lendo positions JOIN assets (ticker, name). findByUserId WHERE quantity > 0; findPosition por (user_id, asset_id). Mapeamento PositionRecord ↔ Position. Sem writes. modules/portfolio/infrastructure/persistence.

- [x] 3.2 `infra:wiring` Registrar módulo portfolio (~1h)
  - **Agent:** `Backend Controller (Rust)`
  - **Prompt:** Adicionar modules/portfolio em mod.rs e lib.rs; injetar PortfolioQuerySqlx + queries no AppState/router pattern existente.

## 4. API — rotas REST read-only

- [x] 4.1 `interface:controller` GET /api/portfolio/* (~2h)
  - **Agent:** `Backend Controller (Rust)`
  - **Prompt:** GET /api/portfolio e GET /api/portfolio/assets/:assetId. JWT owner; userId do token. Mapear Result para 200/404/401. modules/portfolio/interfaces/http.

## 5. Testes backend

- [x] 5.1 `test:unit` Portfolio queries (~2h)
  - **Agent:** `Unit Tests (Rust)`
  - **Prompt:** Testes GetUserPortfolio e GetPositionByAsset com mock PortfolioQuery. Cobrir empty portfolio, single position, not found. Cobertura ≥95% domain+application do módulo portfolio.

- [x] 5.2 `test:e2e` E2E portfolio após trades (~2h)
  - **Agent:** `E2E Tests (Rust)`
  - **Prompt:** E2E: login investor, POST buy ativo ativo, GET /api/portfolio reflete qty e averagePrice, GET /api/portfolio/assets/:id detalhe OK, sell total → GET detail 404, GET portfolio empty.

## 6. Web — entidades, use cases e repository

- [x] 6.1 `interface:entity` PortfolioSummary, PositionDetail (~1h)
  - **Agent:** `Frontend Entity (Leptos)`
  - **Prompt:** Entidades UI PortfolioSummary (totalInvested, positions[]), PositionDetail (assetId, ticker, name, quantity, averagePrice, totalInvested). shared_kernel::Result, sem leptos::* no domain. crate web-leptos/features/portfolio.

- [x] 6.2 `interface:usecase` LoadPortfolio, LoadPosition (~1h)
  - **Agent:** `Frontend UseCase (Leptos)`
  - **Prompt:** LoadPortfolioUseCase e LoadPositionUseCase async orquestrando IPortfolioRepository; retorno Result. Owner via JWT no repository.

- [x] 6.3 `interface:repository` PortfolioHttpRepository (~2h)
  - **Agent:** `Frontend Repository (Leptos)`
  - **Prompt:** reqwest adapter: GET /api/portfolio, GET /api/portfolio/assets/:assetId com JWT. Mapear DTOs API → PortfolioSummary/PositionDetail. #[cfg_attr(wasm32, async_trait(?Send))] no port.

## 7. Web — páginas, shell e integração sell

- [x] 7.1 `interface:page` Carteira /portfolio (~2h)
  - **Agent:** `Frontend Page (Leptos)`
  - **Prompt:** Página /portfolio com Resource; tabela ticker, nome, quantity, averagePrice, totalInvested; total agregado; empty state; links para /portfolio/assets/:id. Rotas privadas no app.rs.

- [x] 7.2 `interface:page` Detalhe posição (~1h)
  - **Agent:** `Frontend Page (Leptos)`
  - **Prompt:** Página /portfolio/assets/:id com PositionDetail; botão "Vender" navega /trades/sell?assetId=:id; not-found state com link /portfolio.

- [x] 7.3 `infra:shell-web` Menu Carteira no shell (~1h)
  - **Agent:** `Config Shared Web (Leptos)`
  - **Prompt:** Menu "Carteira" visível para usuário autenticado (Investor e Admin), link /portfolio. Rotas /portfolio/* no shell privado; guard auth sem admin guard. shell_navigation.rs + sidebar_menu.rs.

- [x] 7.4 `interface:form-web` Sell pre-select assetId (~1h)
  - **Agent:** `Frontend Form (Leptos)`
  - **Prompt:** Estender /trades/sell para ler query param assetId na mount; pré-selecionar ativo e maxQuantity se posição existir; fallback erro se assetId inválido.

## 8. Testes web e fechamento MVP

- [x] 8.1 `test:unit-web` Portfolio UI use cases (~1h)
  - **Agent:** `Frontend UseCase (Leptos)`
  - **Prompt:** Testes unitários LoadPortfolioUseCase e LoadPositionUseCase com mock IPortfolioRepository. Cobrir empty portfolio e not found.

- [x] 8.2 `quality:ci-verify` + `quality:memory-leak` Fechamento MVP (~1h)
  - **Agent:** `Config CI/CD (Rust)`
  - **Prompt:** CI verde Release 1 completa; clippy, test, coverage módulo portfolio; memory check após E2E portfolio. ./scripts/ci-local.sh passa.

## Critérios de aceitação (US-007)

- [x] Dado trades existentes, quando GET portfolio, então posições com qty e averagePrice corretos
- [x] Dado posição inexistente, quando GET portfolio/assets/:id, então 404
- [x] Dado JWT, quando GET portfolio, então apenas carteira do owner

## Critérios de aceitação (US-008)

- [x] Dado `/portfolio`, quando carrega, então exibe posições ou empty state
- [x] Dado detalhe posição, quando clica "Vender", então navega `/trades/sell` com asset pré-selecionado
