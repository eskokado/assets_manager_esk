## Why

Com o catálogo de ativos (EP-002) disponível, a plataforma **Assets Manage** ainda não permite movimentar carteiras — o fluxo central de valor para investidores. O épico EP-003 entrega o bounded context **Trading** (compra, venda e histórico imutável) e materializa **posições de Portfolio** na mesma transação DB, desbloqueando EP-004 (consultas de carteira).

## What Changes

- Criar módulo `trading` em `crates/api/src/modules/trading/` com domínio (Trade AR, VOs TradeSide/Quantity/UnitPrice/TradeTotal, TradeExecutor, SellValidator, PortfolioCalculator), application (BuyAsset, SellAsset, ListTrades, FindTradeById) e infra (TradeSqlxRepository, PositionSqlxRepository)
- Adicionar migrations `trades` e `positions` (PK composta user_id + asset_id)
- Expor `POST /api/trades/buy`, `POST /api/trades/sell`, `GET /api/trades`, `GET /api/trades/:id` com JWT owner
- Enforçar regras RN-009–RN-012: BUY só em ativo ativo, SELL com saldo suficiente, trade imutável, portfolio consistente via TX atômica
- Implementar telas Leptos investidor `/trades`, `/trades/buy`, `/trades/sell` com dropdown de ativos ativos
- Integrar menu "Operações" no shell web para usuários autenticados
- Conectar `AssetCatalogPolicy` (EP-002) com consulta real de posições abertas na desativação de ativos
- Testes unitários (domain + application ≥95%), E2E buy→sell, testes use cases UI e CI verde

## Capabilities

### New Capabilities

- `trading-api`: Endpoints REST buy/sell/list/detail de trades; autorização JWT owner; filtros side/asset/período na listagem
- `trading-persistence`: Schema `trades` (imutável) e `positions` (upsert por user+asset); adapters sqlx com transação compartilhada
- `trading-web`: Telas Leptos `/trades`, `/trades/buy`, `/trades/sell`; entidades UI, use cases e TradeHttpRepository

### Modified Capabilities

- `web-admin-shell`: Item de menu "Operações" para usuários autenticados (Investor e Admin); links para `/trades`, `/trades/buy`, `/trades/sell`
- `assets-api`: AssetCatalogPolicy SHALL consultar posições abertas antes de desativar ativo (integração real com tabela positions)

## Impact

- **Código**: `crates/api/src/modules/trading/**`, migrations em `migrations/`, ajuste em `modules/assets` (AssetCatalogPolicy), `crates/web-leptos` (rotas trading, forms, menu)
- **APIs**: novos endpoints `/api/trades/*`; reutiliza JWT de EP-001 e consulta Assets de EP-002 para validar ativo ativo
- **Dependências**: nenhuma crate nova; reutiliza Money/Result do shared-kernel
- **Downstream**: EP-004 Portfolio consome tabela `positions` para queries de leitura (`GET /api/portfolio`)
- **Documentação**: alinhado a `docs/planning/assets-manage/backlog.md` EP-003, US-005/US-006 e `ddd-tactical-model.md` BC Trading + Portfolio (materialização)
