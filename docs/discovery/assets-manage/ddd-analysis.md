# Análise DDD / Clean Architecture — Assets Manage

**Baseado em**: `docs/discovery/assets-manage/requirements.md`
**Data da análise**: 2026-06-06
**Stack alvo**: Rust (Axum + sqlx + shared-kernel)

## Context Map

```
┌─────────────────────────────────────────────────────────────┐
│                      CONTEXT MAP                            │
│                                                             │
│   ┌──────────┐                                              │
│   │   Auth   │────────── upstream ──────────┐               │
│   │ (identity)│                              │               │
│   └──────────┘                               ▼               │
│        │                              ┌───────────┐          │
│        │                              │  Assets   │          │
│        │                              │ (catalog) │          │
│        │                              └─────┬─────┘          │
│        │                                    │ shared         │
│        │         ┌───────────┐◀─────────────┘ kernel         │
│        └────────▶│ Portfolio │◀──────┐    (AssetId,         │
│                  │ (holdings)│       │     Ticker, Money)    │
│                  └─────┬─────┘       │                       │
│                        │             │                       │
│                        │  events/    │                       │
│                        │  commands   │                       │
│                        ▼             │                       │
│                  ┌───────────┐       │                       │
│                  │  Trading  │───────┘                       │
│                  │ (buy/sell)│  downstream of Auth + Assets  │
│                  └───────────┘                               │
│                                                             │
│  Legenda: ──▶ upstream/downstream   ◀──▶ consulta IDs/VOs   │
└─────────────────────────────────────────────────────────────┘
```

## Bounded Contexts

### BC-001: Auth

**Descrição**: Identidade, credenciais e autorização para acesso ao gerenciamento de ativos.
**Relação**: upstream de Portfolio e Trading; fornece `UserId` e `Role` aos demais contextos.

#### Entities

| Entity | Atributos Principais              | Aggregate Root? |
| ------ | --------------------------------- | --------------- |
| User   | id, name, email, passwordHash, role, status | Sim     |

#### Value Objects

| VO           | Tipo   | Validação                          |
| ------------ | ------ | ---------------------------------- |
| Email        | string | formato RFC, lowercase             |
| Password     | string | política mínima (length, etc.)     |
| HashPassword | string | hash bcrypt — nunca expor plain    |
| UserName     | string | não vazio, max length              |
| Role         | enum   | Admin \| Investor                  |

#### Domain Services

| Service              | Regra                                           |
| -------------------- | ----------------------------------------------- |
| PasswordPolicy       | Valida força da senha no registro/troca         |
| AuthenticationService| Verifica credenciais e emite claims `[app layer]` |

#### Repository Ports

| Repository     | Operações                                              |
| -------------- | ------------------------------------------------------ |
| UserRepository | save, findById, findByEmail, existsByEmail             |

#### Use Cases

| Use Case        | Tipo    | Descrição                              |
| --------------- | ------- | -------------------------------------- |
| RegisterUser    | comando | Cadastra investidor                    |
| LoginUser       | comando | Autentica e retorna token              |
| GetCurrentUser  | query   | Perfil do usuário autenticado          |
| ChangePassword  | comando | **Pós-MVP** `[inferido]`               |

#### DTOs

| DTO              | Direção | Campos                    |
| ---------------- | ------- | ------------------------- |
| RegisterInDTO    | entrada | name, email, password     |
| LoginInDTO       | entrada | email, password           |
| AuthTokenOutDTO  | saída   | accessToken, expiresIn    |
| UserOutDTO       | saída   | id, name, email, role     |

#### Controllers/Endpoints

| Verbo | Rota               | Use Case       |
| ----- | ------------------ | -------------- |
| POST  | /api/auth/register | RegisterUser   |
| POST  | /api/auth/login    | LoginUser      |
| GET   | /api/auth/me       | GetCurrentUser |

---

### BC-002: Assets (Catálogo)

**Descrição**: Cadastro mestre de instrumentos financeiros negociáveis.
**Relação**: shared kernel com Trading/Portfolio via `AssetId`, `Ticker`, `AssetType`; downstream de Auth (Admin-only write).

#### Entities

| Entity | Atributos Principais                         | Aggregate Root? |
| ------ | -------------------------------------------- | --------------- |
| Asset  | id, ticker, name, assetType, currency, active | Sim            |

#### Value Objects

| VO        | Tipo   | Validação                                |
| --------- | ------ | ---------------------------------------- |
| Ticker    | string | uppercase, pattern, único no catálogo      |
| AssetName | string | não vazio                                |
| AssetType | enum   | STOCK, FII, ETF, BOND `[inferido]`       |
| Currency  | string | ISO 4217 (ex.: BRL)                      |
| Money     | decimal + currency | amount >= 0                  |

#### Domain Services

| Service           | Regra                                              |
| ----------------- | -------------------------------------------------- |
| AssetCatalogPolicy| Impede desativação com posições abertas `[inferido]` |

#### Repository Ports

| Repository      | Operações                                    |
| --------------- | -------------------------------------------- |
| AssetRepository | save, findById, findByTicker, findAll, update |

#### Use Cases

| Use Case      | Tipo    | Descrição                         |
| ------------- | ------- | --------------------------------- |
| CreateAsset   | comando | Admin cadastra ativo              |
| UpdateAsset   | comando | Admin atualiza/desativa           |
| FindAssetById | query   | Detalhe do ativo                  |
| ListAssets    | query   | Listagem paginada com filtros     |

#### DTOs

| DTO            | Direção | Campos                                      |
| -------------- | ------- | ------------------------------------------- |
| CreateAssetInDTO | entrada | ticker, name, assetType, currency         |
| AssetOutDTO    | saída   | id, ticker, name, assetType, currency, active |

#### Controllers/Endpoints

| Verbo | Rota            | Use Case      |
| ----- | --------------- | ------------- |
| POST  | /api/assets     | CreateAsset   |
| GET   | /api/assets     | ListAssets    |
| GET   | /api/assets/:id | FindAssetById |
| PUT   | /api/assets/:id | UpdateAsset   |

---

### BC-003: Portfolio

**Descrição**: Visão consolidada das posições do investidor (read model + aggregate derivado).
**Relação**: downstream de Auth e Trading; consulta Assets por ID.

#### Entities

| Entity   | Atributos Principais                              | Aggregate Root? |
| -------- | ------------------------------------------------- | --------------- |
| Portfolio| userId (id lógico), positions[]                   | Sim             |
| Position | assetId, quantity, averagePrice, totalInvested    | Não (child)     |

#### Value Objects

| VO       | Tipo              | Validação              |
| -------- | ----------------- | ---------------------- |
| Quantity | decimal           | > 0 em movimentações   |
| Money    | amount + currency | >= 0                   |

#### Domain Services

| Service              | Regra                                                |
| -------------------- | ---------------------------------------------------- |
| PortfolioCalculator  | Recalcula posição e preço médio após cada trade      |
| PositionValidator    | Garante quantidade não negativa                      |

#### Repository Ports

| Repository         | Operações                              |
| ------------------ | -------------------------------------- |
| PortfolioRepository| findByUserId, save (upsert positions)  |

#### Use Cases

| Use Case           | Tipo  | Descrição                        |
| ------------------ | ----- | -------------------------------- |
| GetUserPortfolio   | query | Carteira do usuário autenticado  |
| GetPositionByAsset | query | Detalhe de posição em um ativo   |

#### DTOs

| DTO              | Direção | Campos                                           |
| ---------------- | ------- | ------------------------------------------------ |
| PortfolioOutDTO  | saída   | userId, positions[], totalInvested `[inferido]`  |
| PositionOutDTO   | saída   | assetId, ticker, quantity, averagePrice          |

#### Controllers/Endpoints

| Verbo | Rota                          | Use Case           |
| ----- | ----------------------------- | ------------------ |
| GET   | /api/portfolio                | GetUserPortfolio   |
| GET   | /api/portfolio/assets/:assetId| GetPositionByAsset |

---

### BC-004: Trading

**Descrição**: Operações de compra e venda vinculadas a usuário e ativo.
**Relação**: downstream de Auth e Assets; atualiza Portfolio via domain service ou evento de aplicação.

#### Entities

| Entity  | Atributos Principais                                      | Aggregate Root? |
| ------- | --------------------------------------------------------- | --------------- |
| Trade   | id, userId, assetId, side, quantity, unitPrice, tradedAt | Sim             |

#### Value Objects

| VO        | Tipo   | Validação                    |
| --------- | ------ | ---------------------------- |
| TradeSide | enum   | BUY \| SELL                  |
| Quantity  | decimal| > 0                          |
| UnitPrice | Money  | > 0                          |
| TradeTotal| Money  | quantity × unitPrice         |

#### Domain Services

| Service        | Regra                                           |
| -------------- | ----------------------------------------------- |
| TradeExecutor  | Orquestra validação de ativo, saldo e persistência |
| SellValidator  | Quantidade ≤ posição atual                      |

#### Repository Ports

| Repository     | Operações                                      |
| -------------- | ---------------------------------------------- |
| TradeRepository| save, findById, findByUserId, findByUserAndAsset |

#### Use Cases

| Use Case    | Tipo    | Descrição                          |
| ----------- | ------- | ---------------------------------- |
| BuyAsset    | comando | Registra compra e atualiza carteira|
| SellAsset   | comando | Registra venda com validação saldo |
| ListTrades  | query   | Histórico do usuário               |
| FindTradeById | query | Detalhe de transação             |

#### DTOs

| DTO           | Direção | Campos                                      |
| ------------- | ------- | ------------------------------------------- |
| BuyAssetInDTO | entrada | assetId, quantity, unitPrice, tradedAt?     |
| SellAssetInDTO| entrada | assetId, quantity, unitPrice, tradedAt?     |
| TradeOutDTO   | saída   | id, side, assetId, quantity, unitPrice, total |

#### Controllers/Endpoints

| Verbo | Rota              | Use Case      |
| ----- | ----------------- | ------------- |
| POST  | /api/trades/buy   | BuyAsset      |
| POST  | /api/trades/sell  | SellAsset     |
| GET   | /api/trades       | ListTrades    |
| GET   | /api/trades/:id   | FindTradeById |

---

## Mapeamento de Camadas (Clean Architecture)

| Camada         | Artefatos Identificados                          | Skills Rust                          |
| -------------- | ------------------------------------------------ | ------------------------------------ |
| Domain         | Entities, VOs, Domain Services, Repository ports | core-entity-rs, core-value-object-rs, core-domain-service-rs, core-repository-rs |
| Application    | Use Cases, DTOs, Queries                         | core-use-case-rs, core-dto-rs, core-query-cqrs-rs |
| Infrastructure | sqlx adapters, JWT, migrations                   | backend-data-rs, config-sqlx-rs      |
| Interface      | Axum handlers, middleware auth                   | backend-controller-rs                |

## Dependências entre Contexts

| De       | Para     | Tipo                | Dados Compartilhados        |
| -------- | -------- | ------------------- | --------------------------- |
| Trading  | Auth     | downstream          | userId (JWT)                |
| Trading  | Assets   | downstream          | assetId, asset active flag  |
| Trading  | Portfolio| downstream (update) | quantity, averagePrice      |
| Portfolio| Assets   | conformist (read)   | ticker, name para exibição  |
| Assets   | Auth     | downstream          | Admin role para escrita     |

## Recomendações Arquiteturais

1. **Módulos Axum por BC**: `crates/api/src/modules/{auth,assets,portfolio,trading}/` — alinhado a `config-new-module-rs`.
2. **Transação atômica** em BuyAsset/SellAsset: persistir `Trade` + atualizar `Portfolio` na mesma unidade de trabalho (sqlx transaction).
3. **Shared kernel** em `crates/shared-kernel`: `Id`, `Email`, `Money`, `Result`, `Entity`, `UseCase`.
4. **CQRS leve**: Portfolio como projeção recalculada ou materializada após cada trade — evitar recalcular full scan em listagens grandes `[inferido]`.
5. **Auth MVP simplificado**: roles Admin/Investor no JWT; RBAC fino (permissions) fica para evolução pós-MVP.
6. **Inteligência futura**: novo BC `Insights` ou `Advisory` consumindo Portfolio + Assets — não acoplar ao MVP.
