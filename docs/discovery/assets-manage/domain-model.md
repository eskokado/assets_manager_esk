# Modelo de Domínio — Assets Manage

**Fonte**: Especificação funcional (greenfield)
**Data**: 2026-06-06

## Entidades por Bounded Context

### BC-001: Auth

#### User (Aggregate Root)

| Campo         | Tipo        | Obrigatório | VO candidato |
| ------------- | ----------- | ----------- | ------------ |
| id            | UUID        | sim         | Id           |
| name          | string      | sim         | UserName     |
| email         | string      | sim         | Email        |
| password_hash | string      | sim         | HashPassword |
| role          | enum        | sim         | Role         |
| active        | boolean     | sim         | —            |
| created_at    | timestamp   | sim         | —            |

- Relações: 1 User → N Trades, 1 Portfolio
- Regras: email único; senha nunca persistida em plain text

---

### BC-002: Assets

#### Asset (Aggregate Root)

| Campo       | Tipo      | Obrigatório | VO candidato |
| ----------- | --------- | ----------- | ------------ |
| id          | UUID      | sim         | Id           |
| ticker      | string    | sim         | Ticker       |
| name        | string    | sim         | AssetName    |
| asset_type  | enum      | sim         | AssetType    |
| currency    | string    | sim         | Currency     |
| active      | boolean   | sim         | —            |
| created_at  | timestamp | sim         | —            |
| updated_at  | timestamp | sim         | —            |

- Relações: 1 Asset → N Trades, N Positions
- Regras: ticker único; inactive bloqueia novas compras

---

### BC-003: Portfolio

#### Portfolio (Aggregate Root)

| Campo     | Tipo     | Obrigatório | VO candidato |
| --------- | -------- | ----------- | ------------ |
| user_id   | UUID     | sim (PK lógico) | Id       |
| updated_at| timestamp| sim         | —            |

#### Position (Entity filha)

| Campo          | Tipo    | Obrigatório | VO candidato |
| -------------- | ------- | ----------- | ------------ |
| asset_id       | UUID    | sim         | Id           |
| quantity       | decimal | sim         | Quantity     |
| average_price  | decimal | sim         | Money        |
| currency       | string  | sim         | Currency     |

- Regras: quantity ≥ 0; recalculada por PortfolioCalculator após trades

---

### BC-004: Trading

#### Trade (Aggregate Root)

| Campo       | Tipo      | Obrigatório | VO candidato |
| ----------- | --------- | ----------- | ------------ |
| id          | UUID      | sim         | Id           |
| user_id     | UUID      | sim         | Id           |
| asset_id    | UUID      | sim         | Id           |
| side        | enum      | sim         | TradeSide    |
| quantity    | decimal   | sim         | Quantity     |
| unit_price  | decimal   | sim         | Money        |
| currency    | string    | sim         | Currency     |
| traded_at   | timestamp | sim         | —            |
| created_at  | timestamp | sim         | —            |

- Relações: FK lógica user_id → User, asset_id → Asset
- Regras: imutável após criação; SELL valida saldo via SellValidator

---

## Diagrama de Aggregates

```
┌─────────────┐
│    User     │  (Auth)
│  AR: User   │
└──────┬──────┘
       │ 1
       │
       │ 1
┌──────▼──────┐       ┌─────────────┐
│  Portfolio  │       │    Asset    │  (Assets)
│  AR: Port.  │       │  AR: Asset  │
│  ┌─────────┐│       └──────┬──────┘
│  │Position ││              │
│  └─────────┘│              │
└──────┬──────┘              │
       │                     │
       │      ┌──────────────▼──────────────┐
       └─────▶│          Trade              │  (Trading)
              │         AR: Trade           │
              │  side, qty, unit_price      │
              └─────────────────────────────┘
```

## Mapeamento Entity ↔ Tabela/Collection

| Entity   | Tabela        | Observações                          |
| -------- | ------------- | ------------------------------------ |
| User     | users         | unique(email), role enum             |
| Asset    | assets        | unique(ticker)                       |
| Portfolio| portfolios    | PK: user_id                          |
| Position | positions     | PK: (user_id, asset_id)              |
| Trade    | trades        | FK user_id, asset_id; index por user |

## Enumerações sugeridas

```text
Role:        ADMIN | INVESTOR
AssetType:   STOCK | FII | ETF | BOND    [inferido]
TradeSide:   BUY | SELL
```

## Índices e performance `[inferido]`

- `trades(user_id, traded_at DESC)` — histórico paginado
- `assets(ticker)` — lookup único
- `positions(user_id)` — carteira do usuário
