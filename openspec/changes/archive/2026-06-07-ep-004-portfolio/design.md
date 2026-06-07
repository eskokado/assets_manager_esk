## Context

**Assets Manage** possui autenticação JWT (EP-001), catálogo de ativos (EP-002) e trading com materialização síncrona de posições (EP-003). A tabela `positions` já reflete quantity e average_price após cada BUY/SELL, mas não há endpoints nem telas de consulta — o investidor opera trades sem visão consolidada da carteira.

**Referências**: `docs/planning/assets-manage/backlog.md` (US-007, US-008), `ddd-tactical-model.md` BC Portfolio, `delivery-profile.md` (Leptos investidor, sem mobile).

**Estado atual**: `Position` e `PortfolioRepository` (write) vivem em `modules/trading`. Não existe módulo `portfolio` nem rotas `/api/portfolio/*`. Shell web tem menu "Operações" mas não "Carteira".

## Goals / Non-Goals

**Goals:**

- Módulo `portfolio` read-only: aggregate Portfolio → queries → sqlx read adapter → rotas HTTP
- Investidor consulta carteira própria com posições enriquecidas (ticker, nome do asset via JOIN)
- Detalhe de posição por assetId com totalInvested calculado (quantity × averagePrice)
- Telas Leptos `/portfolio` e `/portfolio/assets/:id` com empty state e link "Vender" → `/trades/sell?assetId=...`
- Menu "Carteira" no shell para usuários autenticados
- Cobertura ≥95% queries domain+application; E2E buy → GET portfolio reflete posição; CI verde Release 1

**Non-Goals:**

- Escrita ou recálculo de posições (permanece em Trading EP-003)
- Cotações de mercado, P/L unrealized, gráficos, export CSV
- Admin visualizar carteira de outros usuários
- Mobile
- Nova migration ou alteração de schema `positions`

## Decisions

### 1. Módulo Axum `modules/portfolio` separado do Trading

**Decisão**: BC Portfolio como módulo read-only em `modules/portfolio`. Writes permanecem em `modules/trading` (PortfolioRepository upsert).

**Rationale**: Separação CQRS clara; EP-004 = queries + UI; ddd-tactical-model trata Portfolio como BC distinto.

**Alternativa rejeitada**: Adicionar queries em `modules/trading` — mistura write/read no mesmo módulo e dificulta evolução futura (eventos, projeções).

### 2. Reutilizar `Position` do trading via shared type ou re-export

**Decisão**: Portfolio module define aggregate `Portfolio` (userId, positions[]) e reutiliza struct `Position` de `trading::domain` (ou extrai para sub-módulo shared interno se necessário). Read adapter consulta `positions` JOIN `assets`.

**Rationale**: Evita duplicação de VOs Quantity/UnitPrice; mesma tabela, mesma semântica.

**Alternativa rejeitada**: Duplicar entidade Position em portfolio — drift de invariantes.

### 3. PortfolioQueryPort read-only

**Decisão**: Trait `PortfolioQuery` com `findByUserId(userId) -> Portfolio` e `findPosition(userId, assetId) -> Option<Position>`. Adapter `PortfolioQuerySqlx` com SQL JOIN assets (ticker, name).

**Rationale**: CQRS; port distinto do write `PortfolioRepository` em trading.

### 4. Enriquecimento conformist com Assets

**Decisão**: Queries retornam DTOs `PositionOut` com `assetTicker`, `assetName` via JOIN sqlx. Se asset deletado/inativo, ainda exibir posição com ticker fallback (id truncado ou "—").

**Rationale**: RN-009; Open Host Service Assets → Portfolio conformist.

### 5. GET position inexistente → 404

**Decisão**: `GET /api/portfolio/assets/:assetId` retorna 404 quando não há posição ou quantity = 0 para o owner.

**Rationale**: Backlog US-007 critério "404 ou qty zero"; escolhemos 404 explícito para detalhe.

**Alternativa rejeitada**: 200 com qty zero — confunde com posição fechada vs nunca existiu.

### 6. GET portfolio sem posições → 200 empty array

**Decisão**: `GET /api/portfolio` retorna `{ positions: [], totalInvested: "0" }` quando usuário sem holdings.

**Rationale**: Empty state na UI; distinto de 404.

### 7. Estrutura web Leptos

**Decisão**: `PortfolioSummary`, `PositionDetail` + `IPortfolioRepository` (reqwest GET) + LoadPortfolio/LoadPosition use cases; páginas SSR com Resource; detalhe inclui botão "Vender" navegando para `/trades/sell?assetId={uuid}`.

**Rationale**: Skills `frontend-*-leptos`; US-008 critério de aceitação.

### 8. Sell form query param

**Decisão**: `/trades/sell` lê `assetId` de query string na mount; se presente e válido, pré-seleciona dropdown e carrega maxQuantity.

**Rationale**: Fluxo posição → venda sem re-seleção manual.

## Risks / Trade-offs

| Risco | Mitigação |
|-------|-----------|
| Posição stale vs último trade | Positions materializadas na TX de EP-003; read eventual apenas entre commit e GET |
| JOIN assets falha se asset removido | FK asset_id em positions impede delete com posição; fallback ticker se inativo |
| Duplicação Position entity | Re-export de trading::domain::Position no portfolio module |
| Admin vê carteira própria no MVP | Documentado; multi-tenant admin deferido |

## Migration Plan

1. Implementar módulo portfolio backend + rotas (sem migration)
2. Deploy API com novos endpoints (backward compatible)
3. Deploy web com rotas `/portfolio/*` e menu Carteira
4. Rollback: remover rotas portfolio; positions table intacta para trading

## Open Questions

- Nenhuma bloqueante para MVP. Posição com quantity=0: manter row ou filtrar na query? **Decisão**: filtrar `quantity > 0` na listagem; detalhe 404 se zero.
