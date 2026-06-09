## Why

Com EP-003 (Trading) entregue, posições são materializadas na tabela `positions` a cada compra/venda, mas o investidor ainda não consegue **consultar a carteira consolidada** — pré-requisito para decidir vendas parciais e fechar o MVP Release 1. EP-004 entrega o bounded context **Portfolio** em modo leitura (queries + UI), sem duplicar lógica de escrita já existente no Trading.

## What Changes

- Criar módulo `portfolio` em `crates/api/src/modules/portfolio/` com aggregate Portfolio (read model), queries CQRS e adapter sqlx de leitura sobre `positions`
- Expor `GET /api/portfolio` (carteira do JWT owner com posições enriquecidas ticker/nome) e `GET /api/portfolio/assets/:assetId` (detalhe de posição)
- Enforçar regras RN-007–RN-009: apenas owner autenticado; posição inexistente retorna 404; empty state quando sem posições
- Implementar telas Leptos `/portfolio` (listagem) e `/portfolio/assets/:id` (detalhe com atalho "Vender")
- Adicionar menu **Carteira** no shell web para usuários autenticados (Investor e Admin)
- Estender `/trades/sell` para aceitar `assetId` pré-selecionado via query string a partir do detalhe da posição
- Testes unitários (queries ≥95%), E2E buy → GET portfolio, testes use cases UI e CI verde (fechamento MVP)

## Capabilities

### New Capabilities

- `portfolio-api`: Endpoints REST read-only `GET /api/portfolio` e `GET /api/portfolio/assets/:assetId`; autorização JWT owner; enriquecimento conformist com catálogo Assets (ticker, nome)
- `portfolio-web`: Telas Leptos `/portfolio` e `/portfolio/assets/:id`; entidades UI PortfolioSummary/PositionDetail, use cases LoadPortfolio/LoadPosition e PortfolioHttpRepository

### Modified Capabilities

- `web-admin-shell`: Item de menu "Carteira" para usuários autenticados (Investor e Admin); rotas `/portfolio/*` no shell privado
- `trading-web`: Rota `/trades/sell` SHALL aceitar query param `assetId` para pré-selecionar ativo ao navegar a partir do detalhe de posição

## Impact

- **Código**: `crates/api/src/modules/portfolio/**`, wiring em `lib.rs`/`modules/mod.rs`, `crates/web-leptos` (feature portfolio, rotas, menu, ajuste sell page)
- **APIs**: novos endpoints read-only `/api/portfolio/*`; reutiliza JWT de EP-001 e JOIN/read com `assets` de EP-002
- **Persistência**: nenhuma migration nova — leitura da tabela `positions` materializada por EP-003
- **Dependências**: nenhuma crate nova; reutiliza `Quantity`, `UnitPrice`, `Money`, `Result` do shared-kernel
- **Upstream**: EP-003 Trading (positions materializadas); EP-002 Assets (enriquecimento ticker/nome)
- **Documentação**: alinhado a `docs/planning/assets-manage/backlog.md` EP-004, US-007/US-008 e `ddd-tactical-model.md` BC Portfolio
