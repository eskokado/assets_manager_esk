# Modelo Tático — Assets Manage

**Baseado em**: `docs/discovery/assets-manage/` + `ddd-strategic-model.md`
**Data da modelagem**: 2026-06-06
**Stack**: Rust (Axum + sqlx)

---

## BC Auth — Identidade e Acesso

**Subdomínio**: Identidade e Acesso (Generic)

### Domínio (backend)

#### Value Objects

| VO | Validação |
| -- | --------- |
| Email | RFC format, lowercase, trim |
| Password | Política mínima (length ≥ 8 `[inferido]`) |
| HashPassword | Wrapper opaco — nunca logar valor |
| UserName | Não vazio, max 100 chars |
| Role | Enum: `Admin` \| `Investor` |

#### Entities

| Entity | Atributos | AR? |
| ------ | --------- | --- |
| User | id, name, email, passwordHash, role, active, createdAt | Sim |

#### Aggregates

```
User [AR]
```

#### Domain Services

| Service | Regra |
| ------- | ----- |
| PasswordPolicy | Valida senha no registro |
| UserRegistrationPolicy | E-mail único; role padrão Investor |

#### Domain Events

| Evento | Quando | Consumidor |
| ------ | ------ | ---------- |
| — | Sem eventos cross-context no MVP | — |

#### Repository Ports

| Port | Operações |
| ---- | --------- |
| UserRepository | save, findById, findByEmail, existsByEmail |

#### Application (Use Cases / Queries)

| Caso | Tipo | Descrição |
| ---- | ---- | --------- |
| RegisterUser | comando | Cria investidor |
| LoginUser | comando | Valida credenciais, emite JWT |
| GetCurrentUser | query | Perfil autenticado |

### Superfícies de entrega

| API REST | Web admin / investidor | Mobile |
| -------- | ---------------------- | ------ |
| Sim | Sim | Não |

### Apresentação — Web

| Tela / rota | Persona | Ação principal | Endpoints API | Formulário? |
| ----------- | ------- | ---------------- | ------------- | ----------- |
| `/login` | todos | autenticar | POST `/api/auth/login` | Sim |
| `/register` | investidor | criar conta | POST `/api/auth/register` | Sim |
| `/profile` | autenticado | ver perfil | GET `/api/auth/me` | Não |

**Navegação e shell**

- Rotas públicas: `/login`, `/register`
- Rotas privadas: redirect para `/login` sem token
- Menu: item "Perfil" visível se autenticado
- Admin: sem telas extras de auth no MVP (role vem do token)

**Estado e erros**

- Token JWT em storage seguro (httpOnly cookie ou memory `[inferido]`)
- Erros 401 → logout + redirect login
- Erros 422/400 → mensagens inline no formulário

**Entidades de apresentação (cliente)**

| Entidade UI | Campos principais | Origem |
| ----------- | ----------------- | ------ |
| AuthSession | accessToken, expiresIn, userId, role | login response |
| UserProfile | id, name, email, role | GET `/api/auth/me` |

**Use cases de apresentação**

| Use case UI | Repository | Observação |
| ----------- | ---------- | ---------- |
| LoginUseCase | IAuthRepository | Orquestra login + persistência de sessão |
| RegisterUseCase | IAuthRepository | Validação mínima de campos no cliente |
| LoadProfileUseCase | IAuthRepository | Carrega perfil na rota `/profile` |

---

## BC Assets — Catálogo de Ativos

**Subdomínio**: Catálogo de Ativos (Supporting)

### Domínio (backend)

#### Value Objects

| VO | Validação |
| -- | --------- |
| Ticker | Uppercase, pattern alfanumérico, único |
| AssetName | Não vazio |
| AssetType | Enum: STOCK, FII, ETF, BOND |
| Currency | ISO 4217 (default BRL) |

#### Entities

| Entity | Atributos | AR? |
| ------ | --------- | --- |
| Asset | id, ticker, name, assetType, currency, active, createdAt, updatedAt | Sim |

#### Aggregates

```
Asset [AR]
```

#### Domain Services

| Service | Regra |
| ------- | ----- |
| AssetCatalogPolicy | Bloqueia desativação se posição aberta existe `[inferido]` |

#### Domain Events

| Evento | Quando | Consumidor |
| ------ | ------ | ---------- |
| AssetDeactivated | Admin desativa ativo | Trading (rejeita novas compras) — validação síncrona no MVP |

#### Repository Ports

| Port | Operações |
| ---- | --------- |
| AssetRepository | save, findById, findByTicker, findAll (paginated), update |

#### Application (Use Cases / Queries)

| Caso | Tipo | Descrição |
| ---- | ---- | --------- |
| CreateAsset | comando | Admin cadastra ativo |
| UpdateAsset | comando | Admin edita/desativa |
| FindAssetById | query | Detalhe |
| ListAssets | query | Listagem paginada + filtros |

### Superfícies de entrega

| API REST | Web admin | Mobile |
| -------- | --------- | ------ |
| Sim | Sim | Não |

### Apresentação — Web admin

| Tela / rota | Persona | Ação principal | Endpoints API | Formulário? |
| ----------- | ------- | ---------------- | ------------- | ----------- |
| `/admin/assets` | admin | listar ativos | GET `/api/assets` | Não |
| `/admin/assets/new` | admin | cadastrar | POST `/api/assets` | Sim |
| `/admin/assets/:id/edit` | admin | editar/desativar | GET/PUT `/api/assets/:id` | Sim |

**Navegação e shell**

- Menu: "Ativos" visível apenas se `role === Admin`
- Guard: rotas `/admin/*` exigem Admin; 403 → mensagem de acesso negado

**Estado e erros**

- Listagem com paginação e filtro por tipo/status
- Erro de ticker duplicado → campo ticker no formulário

**Entidades de apresentação (cliente)**

| Entidade UI | Campos principais | Origem |
| ----------- | ----------------- | ------ |
| AssetListItem | id, ticker, name, assetType, active | GET `/api/assets` |
| AssetForm | ticker, name, assetType, currency, active | create/edit |

**Use cases de apresentação**

| Use case UI | Repository |
| ----------- | ---------- |
| ListAssetsUseCase | IAssetRepository |
| LoadAssetUseCase | IAssetRepository |
| SaveAssetUseCase | IAssetRepository |

> Investidor consome catálogo **somente leitura** via listagem em fluxo de Trading (sem rota admin).

---

## BC Portfolio — Carteira

**Subdomínio**: Carteira (Core)

### Domínio (backend)

#### Value Objects

| VO | Validação |
| -- | --------- |
| Quantity | Decimal > 0 em movimentações; ≥ 0 em posição |
| Money | amount ≥ 0, currency ISO |

#### Entities

| Entity | Atributos | AR? |
| ------ | --------- | --- |
| Portfolio | userId, updatedAt | Sim |
| Position | assetId, quantity, averagePrice, currency | Não (filha) |

#### Aggregates

```
Portfolio [AR] → Position[]
```

#### Domain Services

| Service | Regra |
| ------- | ----- |
| PortfolioCalculator | Recalcula quantity e averagePrice após BUY; reduz quantity após SELL |
| PositionValidator | quantity nunca negativa |

#### Domain Events

| Evento | Quando | Consumidor |
| ------ | ------ | ---------- |
| — | Atualização síncrona via Trading no MVP | — |

#### Repository Ports

| Port | Operações |
| ---- | --------- |
| PortfolioRepository | findByUserId, save (upsert positions) |

#### Application (Use Cases / Queries)

| Caso | Tipo | Descrição |
| ---- | ---- | --------- |
| GetUserPortfolio | query | Carteira do JWT owner |
| GetPositionByAsset | query | Posição em um ativo |

### Superfícies de entrega

| API REST | Web investidor | Mobile |
| -------- | -------------- | ------ |
| Sim | Sim | Não |

### Apresentação — Web investidor

| Tela / rota | Persona | Ação principal | Endpoints API | Formulário? |
| ----------- | ------- | ---------------- | ------------- | ----------- |
| `/portfolio` | investidor | ver carteira | GET `/api/portfolio` | Não |
| `/portfolio/assets/:assetId` | investidor | detalhe posição | GET `/api/portfolio/assets/:assetId` | Não |

**Navegação e shell**

- Menu: "Carteira" visível para Investidor e Admin (admin vê própria carteira no MVP)
- Link da posição → detalhe + atalho para vender

**Estado e erros**

- Empty state quando sem posições
- Exibir ticker/nome enriquecido (join conformist com Assets na API)

**Entidades de apresentação (cliente)**

| Entidade UI | Campos principais | Origem |
| ----------- | ----------------- | ------ |
| PortfolioSummary | totalInvested, positions[] | GET `/api/portfolio` |
| PositionDetail | assetId, ticker, quantity, averagePrice, totalInvested | GET `/api/portfolio/assets/:id` |

**Use cases de apresentação**

| Use case UI | Repository |
| ----------- | ---------- |
| LoadPortfolioUseCase | IPortfolioRepository |
| LoadPositionUseCase | IPortfolioRepository |

---

## BC Trading — Negociação

**Subdomínio**: Negociação (Core)

### Domínio (backend)

#### Value Objects

| VO | Validação |
| -- | --------- |
| TradeSide | BUY \| SELL |
| Quantity | Decimal > 0 |
| UnitPrice | Money > 0 |
| TradeTotal | quantity × unitPrice |

#### Entities

| Entity | Atributos | AR? |
| ------ | --------- | --- |
| Trade | id, userId, assetId, side, quantity, unitPrice, currency, tradedAt, createdAt | Sim |

#### Aggregates

```
Trade [AR]   (imutável após persistência)
```

#### Domain Services

| Service | Regra |
| ------- | ----- |
| TradeExecutor | Valida ativo ativo (BUY), saldo (SELL), persiste trade + atualiza portfolio |
| SellValidator | quantity solicitada ≤ posição atual |

#### Domain Events

| Evento | Quando | Consumidor |
| ------ | ------ | ---------- |
| TradeExecuted | Após BUY ou SELL confirmado | Portfolio (sync no MVP) |

#### Repository Ports

| Port | Operações |
| ---- | --------- |
| TradeRepository | save, findById, findByUserId (paginated), findByUserAndAsset |

#### Application (Use Cases / Queries)

| Caso | Tipo | Descrição |
| ---- | ---- | --------- |
| BuyAsset | comando | Compra + atualiza portfolio (TX) |
| SellAsset | comando | Venda + atualiza portfolio (TX) |
| ListTrades | query | Histórico do usuário |
| FindTradeById | query | Detalhe (owner ou admin) |

### Superfícies de entrega

| API REST | Web investidor | Mobile |
| -------- | -------------- | ------ |
| Sim | Sim | Não |

### Apresentação — Web investidor

| Tela / rota | Persona | Ação principal | Endpoints API | Formulário? |
| ----------- | ------- | ---------------- | ------------- | ----------- |
| `/trades` | investidor | histórico | GET `/api/trades` | Não |
| `/trades/buy` | investidor | registrar compra | POST `/api/trades/buy` | Sim |
| `/trades/sell` | investidor | registrar venda | POST `/api/trades/sell` | Sim |

**Navegação e shell**

- Menu: "Operações" ou "Negociar"
- Fluxo: listar ativos ativos → selecionar → comprar
- Fluxo venda: partir de `/portfolio` com asset pré-selecionado

**Estado e erros**

- Erro saldo insuficiente (SELL) → mensagem clara com qty disponível
- Erro ativo inativo (BUY) → bloqueio no select de ativos
- Sucesso → redirect `/portfolio` ou `/trades`

**Entidades de apresentação (cliente)**

| Entidade UI | Campos principais | Origem |
| ----------- | ----------------- | ------ |
| TradeListItem | id, side, ticker, quantity, unitPrice, total, tradedAt | GET `/api/trades` |
| BuyTradeForm | assetId, quantity, unitPrice, tradedAt? | POST buy |
| SellTradeForm | assetId, quantity, unitPrice, tradedAt?, maxQuantity | POST sell |

**Use cases de apresentação**

| Use case UI | Repository |
| ----------- | ---------- |
| ListTradesUseCase | ITradeRepository |
| BuyAssetUseCase | ITradeRepository |
| SellAssetUseCase | ITradeRepository |
| LoadActiveAssetsUseCase | IAssetRepository | Para dropdown de compra |

---

## Shared Kernel (`crates/shared-kernel`)

Artefatos compartilhados entre BCs (sem regras de negócio específicas):

| Artefato | Uso |
| -------- | --- |
| Id | UUID wrapper |
| Email | Auth |
| Money | Trading, Portfolio, Assets |
| Result | Todas as camadas |
| Entity | Base trait/struct |
| UseCase | Trait de application services |

---

## Invariantes cross-aggregate (MVP)

| Invariante | Onde enforced |
| ---------- | ------------- |
| BUY só em ativo `active` | TradeExecutor + consulta Assets |
| SELL qty ≤ posição | SellValidator + PortfolioRepository |
| Trade imutável | Sem update/delete no repository |
| Portfolio consistente com trades | Mesma sqlx transaction em BuyAsset/SellAsset |
| Admin-only write em Assets | Axum middleware + role claim |
