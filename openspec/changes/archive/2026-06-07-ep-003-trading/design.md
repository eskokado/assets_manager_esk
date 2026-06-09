## Context

**Assets Manage** possui workspace Rust (EP-000), autenticação JWT (EP-001) e catálogo de ativos com CRUD Admin (EP-002). Investidores podem listar ativos, mas não registrar operações — o core de negócio está bloqueado. O backlog EP-003 define o bounded context `trading` com compra/venda imutável e atualização síncrona de posições na mesma transação sqlx.

**Referências**: `docs/planning/assets-manage/backlog.md` (US-005, US-006), `ddd-tactical-model.md` BC Trading + Portfolio (materialização), `delivery-profile.md` (Leptos investidor, sem mobile).

**Estado atual**: Não existe módulo `trading` nem tabelas `trades`/`positions`. `AssetCatalogPolicy` em EP-002 pode ter stub para posições — EP-003 implementa consulta real.

## Goals / Non-Goals

**Goals:**

- Módulo `trading` completo inside-out: VOs → Trade AR → domain services → use cases/queries → sqlx → rotas HTTP
- Materializar posições (`positions`) dentro de BuyAsset/SellAsset na mesma TX sqlx
- Investidor registra BUY/SELL e consulta histórico paginado próprio
- Validar ativo ativo (BUY), saldo suficiente (SELL), trade imutável após persistência
- Telas Leptos `/trades`, `/trades/buy`, `/trades/sell` com dropdown de ativos ativos
- Cobertura ≥95% domain+application do módulo trading; E2E buy → verify position → sell parcial → sell excesso falha
- CI verde incluindo trading backend + Leptos + memory check

**Non-Goals:**

- Estorno/reversão de trades, ordens pendentes, ordem a mercado com cotação externa
- API de portfolio read-only (EP-004: `GET /api/portfolio`)
- Eventos assíncronos TradeExecuted — integração Portfolio síncrona no MVP
- Mobile
- Admin audit de trades de outros usuários no MVP (FindTradeById owner-only; admin deferido)

## Decisions

### 1. Módulo Axum `modules/trading` + positions no mesmo BC

**Decisão**: Bounded context `modules/trading` contém Trade aggregate, domain services e adapters de `trades` + `positions`. Portfolio read queries ficam em EP-004 (`modules/portfolio`).

**Rationale**: Forte acoplamento TX buy/sell ↔ posição; ddd-operational-notes recomenda materialização conjunta no MVP.

**Alternativa rejeitada**: Crate `portfolio` separado para writes — duplicaria TX boundary e complexidade.

### 2. VOs e Money do shared-kernel

**Decisão**: `TradeSide` (BUY|SELL), `Quantity` (>0), `UnitPrice` (Money >0), `TradeTotal` (qty × price) no domain trading; reutilizar `Money` do shared-kernel.

**Rationale**: RN-011; consistência com Assets.

### 3. Trade imutável — sem update/delete

**Decisão**: `TradeRepository` expõe apenas `save`, `findById`, `findByUserId` (paginated). Sem métodos de update ou delete.

**Rationale**: RN-010 auditoria; correções via estorno fora do MVP.

### 4. Transação atômica BuyAsset/SellAsset

**Decisão**: Use cases recebem `TransactionManager` (ou pool + `begin` explícito) e executam: validar → persist trade → upsert position → commit. Rollback em qualquer falha.

**Rationale**: RN-009, invariante "Portfolio consistente com trades"; RNF-007 concorrência.

**Alternativa rejeitada**: Saga/eventual consistency — over-engineering para MVP monolith.

### 5. PortfolioCalculator — preço médio ponderado

**Decisão**: Domain service `PortfolioCalculator`:
- BUY: `new_qty = old_qty + buy_qty`; `new_avg = (old_qty * old_avg + buy_qty * buy_price) / new_qty`
- SELL: reduz quantity; average_price mantém-se até qty=0 (posição removida ou zerada)

**Rationale**: RN-008 inferido; ddd-tactical-model PortfolioCalculator.

### 6. Validações cross-BC

**Decisão**:
- `TradeExecutor` consulta `AssetRepository.findById` e rejeita BUY se `active == false`
- `SellValidator` consulta `PortfolioRepository` e rejeita se `requested_qty > position.quantity`

**Rationale**: Open Host Service Assets → Trading; RN-005, RN-012.

### 7. Estrutura web Leptos investidor

**Decisão**: `TradeListItem`, `BuyTradeForm`, `SellTradeForm` + `ITradeRepository` (reqwest) + List/Buy/Sell/LoadActiveAssets use cases; páginas SSR com Resource para histórico e signals nos forms.

**Rationale**: Skills `frontend-*-leptos`; rotas `/trades/*`; SellTradeForm inclui `maxQuantity` da posição.

### 8. Menu "Operações" no shell

**Decisão**: Item de menu visível para qualquer usuário autenticado (Investor e Admin); sublinks ou páginas separadas buy/sell/histórico.

**Rationale**: US-006 navegação; ddd-tactical-model "Operações" ou "Negociar".

### 9. AssetCatalogPolicy — integração real

**Decisão**: Atualizar `AssetCatalogPolicy` em `modules/assets` para consultar `PortfolioRepository` (ou query sqlx direta) antes de desativar; bloquear se posição com qty > 0.

**Rationale**: EP-002 deixou stub; EP-003 cria tabela positions.

## Risks / Trade-offs

| Risco | Mitigação |
|-------|-----------|
| Race condition em SELL concorrente | TX serializable ou SELECT FOR UPDATE na position row |
| Posição órfã se TX falhar após save trade | TX única engloba trade + position; teste E2E rollback |
| Dropdown buy com muitos ativos | Reutilizar GET /api/assets?active=true paginado; MVP aceita lista completa |
| EP-004 depende de positions corretas | E2E valida posição após buy; unit tests PortfolioCalculator |
| Sell form sem posição pré-carregada | LoadActiveAssets + LoadPosition para maxQuantity no sell |

## Migration Plan

1. Aplicar migrations `trades` e `positions` em Postgres (dev/CI)
2. Deploy API com rotas `/api/trades/*` protegidas por JWT
3. Atualizar AssetCatalogPolicy para consultar positions
4. Deploy web-leptos com rotas `/trades/*` e menu "Operações"
5. Validar E2E: login → buy → GET trades → sell parcial → sell excesso 422

**Rollback**: Reverter migrations se sem dados de produção; remover rotas trading do shell. Positions vazias não afetam EP-002.

## Open Questions

- `traded_at` default now() vs campo opcional no form — preferir default server-side com override opcional no DTO
- Filtro período em ListTrades: query params `from`/`to` ISO8601 — alinhar com backlog US-005
- Admin FindTradeById de outro user — deferir para release futura; MVP owner-only
