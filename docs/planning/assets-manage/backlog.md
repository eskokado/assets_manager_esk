# Backlog — Assets Manage

**Baseado em**: `docs/discovery/assets-manage/requirements.md`
**Análise DDD**: `docs/modeling/assets-manage/ddd-strategic-model.md` + `ddd-tactical-model.md`
**Data**: 2026-06-06
**Total**: 5 épicos (4 bounded contexts + 1 técnico), 9 stories, ~128 tasks
**Perfil de entrega**: `docs/planning/assets-manage/delivery-profile.md`
**Stack**: Rust (Axum + sqlx) · Leptos SSR · Nenhum mobile

## Roadmap

### Release 0 — Bootstrap

- EP-000 [TECH]: Setup projeto, shared kernel, Postgres, Docker, CI/CD, shell Leptos

### Release 1 — MVP

- EP-001: Auth — Identidade e Acesso
- EP-002: Assets — Catálogo de Ativos
- EP-003: Trading — Compra e Venda
- EP-004: Portfolio — Carteira do Investidor

---

## EP-000 [TECH]: Bootstrap e Infraestrutura

**Descrição**: Workspace Cargo (api + shared-kernel + web-leptos), Postgres, migrations sqlx, Docker produção, CI Rust, shell admin Leptos
**Tamanho**: M
**Dependências**: nenhuma
**Release**: 0
**OpenSpec**: `bootstrap-assets-manage`

### US-000: Setup do Projeto

> Como desenvolvedor, quero o monorepo Rust configurado com API, banco, Docker, CI e shell web, para implementar os bounded contexts com entrega contínua desde o início.

**Prioridade**: Must
**Estimativa**: 8 pontos
**Ref**: RNF-001, RNF-005

**Critérios de Aceitação**:

- [ ] `cargo build` e `cargo test -p shared-kernel` passam
- [ ] `GET /health` retorna 200
- [ ] Postgres sobe via docker-compose; migrations aplicam
- [ ] Pipeline CI executa clippy, test, coverage shared-kernel
- [ ] Shell Leptos exibe layout com sidebar, topbar e dashboard vazio

**Tasks**:

- [ ] `infra:fullstack` Orquestrar bootstrap Rust + Leptos (~1h)
  - **Agent:** `Config Project Full-Stack`
  - **Prompt:** "Bootstrap Assets Manage: backend Rust Axum (config-project-rs), frontend Leptos SSR (config-project-leptos), Postgres sqlx, docker e cicd no EP-000. OpenSpec: bootstrap-assets-manage."

- [ ] `infra:setup` Bootstrap workspace Cargo (~2h)
  - **Agent:** `Config Project (Rust)`
  - **Prompt:** "Inicialize workspace com crates/shared-kernel e crates/api (Axum). docker-compose Postgres, .env.example, health module."

- [ ] `infra:setup` Bootstrap frontend Leptos (~2h)
  - **Agent:** `Config Project (Leptos)`
  - **Prompt:** "Crie crate web-leptos com cargo-leptos, Tailwind v4, proxy para API local."

- [ ] `infra:shell-web` Shell admin Tailwind (~1h)
  - **Agent:** `Config Shared Web (Leptos)`
  - **Prompt:** "Configure shell: sidebar colapsável, topbar, rodapé, dashboard vazio e rotas de referência."

- [ ] `domain:shared` Shared kernel (~2h)
  - **Agent:** `Config Shared Core (Rust)`
  - **Prompt:** "Crie Entity, ValueObject, Result, UseCase, Id, Email, Money em shared-kernel com testes unitários."

- [ ] `infra:db` Configurar sqlx + migrations (~1h)
  - **Agent:** `Config SQLx (Rust)`
  - **Prompt:** "Configure DATABASE_URL, pool sqlx, pasta migrations/, migrate no startup ou script."

- [ ] `infra:docker` Docker multi-stage produção (~1h)
  - **Agent:** `Config Docker (Rust)`
  - **Prompt:** "Dockerfile multi-stage para api e web-leptos; docker-compose.prod.yml."

- [ ] `infra:cicd` Pipeline GitHub Actions (~2h)
  - **Agent:** `Config CI/CD (Rust)`
  - **Prompt:** "CI: fmt, clippy, cargo test, coverage shared-kernel ≥95%. Job memory-check placeholder."

- [ ] `interface:controller` Health check (~30min)
  - **Agent:** `Backend Controller (Rust)`
  - **Prompt:** "GET /health em modules/health retornando status ok."

- [ ] `test:unit` Testes shared-kernel (~1h)
  - **Agent:** `Unit Tests (Rust)`
  - **Prompt:** "Garanta testes Id, Email, Money, Result com cobertura ≥95% em shared-kernel."

- [ ] `test:e2e` E2E health (~1h)
  - **Agent:** `E2E Tests (Rust)`
  - **Prompt:** "Integration test: sobe app, GET /health → 200."

- [ ] `quality:ci-verify` Pipeline verde (~30min)
  - **Agent:** `Config CI/CD (Rust)`
  - **Prompt:** "Abrir PR de bootstrap; corrigir falhas até CI verde."

---

## EP-001: Auth — Identidade e Acesso

**Bounded Context**: BC Auth
**Descrição**: Registro, login JWT, perfil autenticado, roles Admin/Investor
**Tamanho**: M
**Dependências**: EP-000
**Release**: 1
**OpenSpec**: `ep-001-auth`

### US-001: API de autenticação

> Como investidor ou administrador, quero me registrar e autenticar via API, para acessar recursos protegidos com JWT.

**Prioridade**: Must
**Estimativa**: 8 pontos
**Ref**: RF-001, RF-002, RF-003, RF-004, RN-001, RN-003

**Critérios de Aceitação**:

- [ ] Dado e-mail novo, quando POST register, então usuário Investor é criado e retorna 201
- [ ] Dado credenciais válidas, quando POST login, então retorna JWT com userId e role
- [ ] Dado token válido, quando GET me, então retorna perfil sem passwordHash
- [ ] Dado e-mail duplicado, quando register, então retorna erro de negócio
- [ ] Dado senha em texto plano, quando persistido, então apenas hash bcrypt é armazenado

**Tasks**:

- [ ] `domain:vo` VOs Email, Password, HashPassword, UserName, Role (~2h)
  - **Agent:** `Core Value Object (Rust)`
  - **Prompt:** "Crie VOs Email, Password, HashPassword, UserName, Role em modules/auth/domain. Create() retorna Result. Password min 8 chars."

- [ ] `domain:entity` Entidade User (~2h)
  - **Agent:** `Core Entity (Rust)`
  - **Prompt:** "Aggregate User com register() e verify_password(). Role default Investor."

- [ ] `domain:service` PasswordPolicy (~1h)
  - **Agent:** `Core Domain Service (Rust)`
  - **Prompt:** "PasswordPolicy valida força da senha no registro."

- [ ] `domain:repository` Port UserRepository (~1h)
  - **Agent:** `Core Repository (Rust)`
  - **Prompt:** "Trait UserRepository: save, find_by_id, find_by_email, exists_by_email."

- [ ] `app:dto` RegisterIn, LoginIn, AuthTokenOut, UserOut (~1h)
  - **Agent:** `Core DTO (Rust)`
  - **Prompt:** "DTOs serde para register, login, token e perfil."

- [ ] `app:usecase` RegisterUser, LoginUser (~2h)
  - **Agent:** `Core Use Case (Rust)`
  - **Prompt:** "RegisterUser verifica e-mail único, hash bcrypt, persiste. LoginUser valida credenciais, emite JWT."

- [ ] `app:query` GetCurrentUser (~1h)
  - **Agent:** `Core Query CQRS (Rust)`
  - **Prompt:** "GetCurrentUser por userId do JWT retorna UserOut."

- [ ] `infra:persistence` UserSqlxRepository (~2h)
  - **Agent:** `Backend Data (Rust)`
  - **Prompt:** "Adapter sqlx para users; mapeamento UserRecord ↔ User."

- [ ] `infra:migration` Migration users (~1h)
  - **Agent:** `Config SQLx (Rust)`
  - **Prompt:** "Migration users: id, name, email unique, password_hash, role, active, created_at."

- [ ] `interface:controller` Rotas auth (~2h)
  - **Agent:** `Backend Controller (Rust)`
  - **Prompt:** "POST /api/auth/register, POST /api/auth/login, GET /api/auth/me. Middleware JWT."

- [ ] `test:unit` Testes domain + use cases (~2h)
  - **Agent:** `Unit Tests (Rust)`
  - **Prompt:** "Testes VOs, User, RegisterUser, LoginUser com mocks. Cobertura ≥95% domain+application do módulo auth."

- [ ] `test:e2e` E2E register → login → me (~2h)
  - **Agent:** `E2E Tests (Rust)`
  - **Prompt:** "Fluxo HTTP: register, login, GET me com Bearer token."

### US-002: Telas de autenticação (Leptos)

> Como usuário, quero telas de login, registro e perfil, para acessar a plataforma sem usar apenas a API.

**Prioridade**: Must
**Estimativa**: 5 pontos
**Ref**: RF-001, RF-002, RF-003

**Critérios de Aceitação**:

- [ ] Dado usuário em `/login`, quando credenciais corretas, então redireciona para dashboard/carteira
- [ ] Dado usuário em `/register`, quando formulário válido, então conta criada e redireciona login
- [ ] Dado rota privada sem token, quando acessada, então redirect `/login`
- [ ] Dado usuário em `/profile`, quando autenticado, então exibe nome, e-mail e role

### Telas e fluxos (web)

| Rota / tela | Persona | Ação | API | Form? |
|-------------|---------|------|-----|-------|
| `/login` | todos | autenticar | POST `/api/auth/login` | Sim |
| `/register` | investidor | criar conta | POST `/api/auth/register` | Sim |
| `/profile` | autenticado | ver perfil | GET `/api/auth/me` | Não |

**Navegação**: rotas públicas `/login`, `/register`; privadas exigem JWT; menu "Perfil" se autenticado.

**Entidades / use cases UI**: AuthSession, UserProfile → LoginUseCase, RegisterUseCase, LoadProfileUseCase via IAuthRepository.

**Tasks**:

- [ ] `interface:entity` AuthSession, UserProfile (~1h)
  - **Agent:** `Frontend Entity (Leptos)`
  - **Prompt:** "Entidades AuthSession e UserProfile com shared_kernel::Result, sem leptos::* no domain."

- [ ] `interface:usecase` Login, Register, LoadProfile (~2h)
  - **Agent:** `Frontend UseCase (Leptos)`
  - **Prompt:** "Use cases async orquestrando IAuthRepository; retorno Result."

- [ ] `interface:repository` AuthHttpRepository (~2h)
  - **Agent:** `Frontend Repository (Leptos)`
  - **Prompt:** "reqwest adapter: login, register, me. Mapear DTOs → entidades UI."

- [ ] `interface:form-web` Formulários login e register (~2h)
  - **Agent:** `Frontend Form (Leptos)`
  - **Prompt:** "Forms SSR com signals; submit chama use cases; erros inline."

- [ ] `interface:page` Páginas login, register, profile (~2h)
  - **Agent:** `Frontend Page (Leptos)`
  - **Prompt:** "Rotas /login, /register, /profile. Guard redirect se não autenticado."

- [ ] `test:unit-web` Testes use cases auth UI (~1h)
  - **Agent:** `Frontend UseCase (Leptos)`
  - **Prompt:** "Testes unitários LoginUseCase/RegisterUseCase com mock repository."

- [ ] `quality:ci-verify` CI verde EP-001 (~30min)
  - **Agent:** `Config CI/CD (Rust)`
  - **Prompt:** "Garantir CI verde com módulo auth backend + telas Leptos."

- [ ] `quality:memory-leak` Memory check (~30min)
  - **Agent:** `Config CI/CD (Rust)`
  - **Prompt:** "Rodar check-memory-rs.sh após E2E auth; corrigir vazamentos Arc/tasks."

---

## EP-002: Assets — Catálogo de Ativos

**Bounded Context**: BC Assets
**Descrição**: CRUD Admin de instrumentos financeiros; listagem pública autenticada
**Tamanho**: M
**Dependências**: EP-001
**Release**: 1
**OpenSpec**: `ep-002-assets`

### US-003: API de catálogo de ativos

> Como administrador, quero cadastrar e gerenciar ativos via API, para que investidores negociem instrumentos válidos.

**Prioridade**: Must
**Estimativa**: 8 pontos
**Ref**: RF-005, RF-006, RF-007, RF-008, RN-004, RN-005

**Critérios de Aceitação**:

- [ ] Dado Admin autenticado, quando POST asset, então ativo criado com ticker único
- [ ] Dado Investidor, quando POST asset, então retorna 403
- [ ] Dado ativo inativo, quando investidor tenta compra (EP-003), então bloqueado
- [ ] Dado filtros, quando GET assets, então lista paginada

**Tasks**:

- [ ] `domain:vo` Ticker, AssetName, AssetType, Currency (~2h)
  - **Agent:** `Core Value Object (Rust)`
  - **Prompt:** "VOs Ticker (uppercase unique), AssetName, AssetType enum, Currency ISO."

- [ ] `domain:entity` Asset aggregate (~2h)
  - **Agent:** `Core Entity (Rust)`
  - **Prompt:** "Asset AR: create, deactivate. Bloqueia compra se inactive."

- [ ] `domain:service` AssetCatalogPolicy (~1h)
  - **Agent:** `Core Domain Service (Rust)`
  - **Prompt:** "Policy para desativação; consulta posições abertas [inferido]."

- [ ] `domain:repository` AssetRepository port (~1h)
  - **Agent:** `Core Repository (Rust)`
  - **Prompt:** "Trait: save, find_by_id, find_by_ticker, find_all, update."

- [ ] `app:dto` CreateAssetIn, AssetOut (~1h)
  - **Agent:** `Core DTO (Rust)`
  - **Prompt:** "DTOs entrada/saída para asset."

- [ ] `app:usecase` CreateAsset, UpdateAsset (~2h)
  - **Agent:** `Core Use Case (Rust)`
  - **Prompt:** "CreateAsset Admin-only. UpdateAsset permite desativar."

- [ ] `app:query` FindAssetById, ListAssets (~2h)
  - **Agent:** `Core Query CQRS (Rust)`
  - **Prompt:** "Queries paginadas com filtros tipo/status/busca."

- [ ] `infra:persistence` AssetSqlxRepository (~2h)
  - **Agent:** `Backend Data (Rust)`
  - **Prompt:** "Adapter sqlx assets; unique(ticker)."

- [ ] `infra:migration` Migration assets (~1h)
  - **Agent:** `Config SQLx (Rust)`
  - **Prompt:** "Tabela assets: id, ticker unique, name, asset_type, currency, active, timestamps."

- [ ] `interface:controller` Rotas /api/assets (~2h)
  - **Agent:** `Backend Controller (Rust)`
  - **Prompt:** "CRUD REST; POST/PUT exigem role Admin."

- [ ] `test:unit` + `test:e2e` Catálogo (~3h)
  - **Agent:** `Unit Tests (Rust)` / `E2E Tests (Rust)`
  - **Prompt:** "Unit ≥95% domain+app. E2E: admin cria ativo, investor GET lista, investor POST 403."

### US-004: Telas admin de ativos (Leptos)

> Como administrador, quero telas para listar, cadastrar e editar ativos, para manter o catálogo sem usar Postman.

**Prioridade**: Must
**Estimativa**: 5 pontos
**Ref**: RF-005, RF-008

**Critérios de Aceitação**:

- [ ] Dado Admin em `/admin/assets`, quando lista carrega, então exibe ticker, nome, tipo, status
- [ ] Dado Admin em `/admin/assets/new`, quando salva, então ativo aparece na listagem
- [ ] Dado Investidor, quando acessa `/admin/assets`, então 403 ou redirect

### Telas e fluxos (web)

| Rota / tela | Persona | Ação | API | Form? |
|-------------|---------|------|-----|-------|
| `/admin/assets` | admin | listar | GET `/api/assets` | Não |
| `/admin/assets/new` | admin | cadastrar | POST `/api/assets` | Sim |
| `/admin/assets/:id/edit` | admin | editar/desativar | GET/PUT `/api/assets/:id` | Sim |

**Navegação**: menu "Ativos" visível se role Admin; guard `/admin/*`.

**Tasks**:

- [ ] `interface:entity` AssetListItem, AssetForm (~1h)
  - **Agent:** `Frontend Entity (Leptos)`
  - **Prompt:** "Entidades UI para listagem e formulário de asset."

- [ ] `interface:usecase` List, Load, Save asset (~2h)
  - **Agent:** `Frontend UseCase (Leptos)`
  - **Prompt:** "ListAssetsUseCase, LoadAssetUseCase, SaveAssetUseCase."

- [ ] `interface:repository` AssetHttpRepository (~2h)
  - **Agent:** `Frontend Repository (Leptos)`
  - **Prompt:** "HTTP adapter CRUD assets com JWT Admin."

- [ ] `interface:page` Listagem admin (~2h)
  - **Agent:** `Frontend Page (Leptos)`
  - **Prompt:** "Página /admin/assets com Resource e tabela."

- [ ] `interface:form-web` Form create/edit (~2h)
  - **Agent:** `Frontend Form (Leptos)`
  - **Prompt:** "Forms new/edit asset; toggle active; erros de ticker duplicado."

- [ ] `test:unit-web` Use cases asset UI (~1h)
  - **Agent:** `Frontend UseCase (Leptos)`
  - **Prompt:** "Testes SaveAssetUseCase com mock repository."

- [ ] `quality:ci-verify` CI verde EP-002 (~30min)
  - **Agent:** `Config CI/CD (Rust)`
  - **Prompt:** "CI verde com módulo assets completo."

---

## EP-003: Trading — Negociação

**Bounded Context**: BC Trading (+ atualização Portfolio na mesma TX)
**Descrição**: Compra e venda de ativos; histórico imutável; recálculo de posição
**Tamanho**: G
**Dependências**: EP-002
**Release**: 1
**OpenSpec**: `ep-003-trading`

### US-005: API de compra e venda

> Como investidor, quero registrar compras e vendas via API, para movimentar minha carteira de forma auditável.

**Prioridade**: Must
**Estimativa**: 13 pontos
**Ref**: RF-011, RF-012, RF-013, RF-014, RN-009, RN-010, RN-011, RN-012

**Critérios de Aceitação**:

- [ ] Dado ativo ativo, quando BUY qty>0 e price>0, então trade criado e posição aumentada
- [ ] Dado saldo insuficiente, quando SELL, então erro de negócio
- [ ] Dado ativo inativo, quando BUY, então rejeitado
- [ ] Dado usuário autenticado, quando GET trades, então histórico paginado próprio
- [ ] Dado BUY e update portfolio, quando falha parcial, então rollback TX

**Tasks**:

- [ ] `domain:vo` TradeSide, Quantity, UnitPrice, TradeTotal (~2h)
  - **Agent:** `Core Value Object (Rust)`
  - **Prompt:** "VOs TradeSide BUY|SELL, Quantity>0, UnitPrice Money>0."

- [ ] `domain:entity` Trade aggregate (~2h)
  - **Agent:** `Core Entity (Rust)`
  - **Prompt:** "Trade AR imutável após create."

- [ ] `domain:service` TradeExecutor, SellValidator (~2h)
  - **Agent:** `Core Domain Service (Rust)`
  - **Prompt:** "TradeExecutor valida ativo ativo e saldo; orquestra portfolio update."

- [ ] `domain:service` PortfolioCalculator (~2h)
  - **Agent:** `Core Domain Service (Rust)`
  - **Prompt:** "Recalcula quantity e average_price após BUY; reduz após SELL."

- [ ] `domain:repository` TradeRepository, PortfolioRepository ports (~1h)
  - **Agent:** `Core Repository (Rust)`
  - **Prompt:** "Ports trade + portfolio (upsert position)."

- [ ] `app:dto` BuyIn, SellIn, TradeOut (~1h)
  - **Agent:** `Core DTO (Rust)`
  - **Prompt:** "DTOs buy/sell/trade out."

- [ ] `app:usecase` BuyAsset, SellAsset (~3h)
  - **Agent:** `Core Use Case (Rust)`
  - **Prompt:** "Use cases em sqlx transaction: save trade + update portfolio."

- [ ] `app:query` ListTrades, FindTradeById (~2h)
  - **Agent:** `Core Query CQRS (Rust)`
  - **Prompt:** "Queries histórico por userId; filtros side/asset/período."

- [ ] `infra:persistence` Trade + Position adapters (~3h)
  - **Agent:** `Backend Data (Rust)`
  - **Prompt:** "Sqlx repos trades e positions; TX compartilhada."

- [ ] `infra:migration` trades + positions (~1h)
  - **Agent:** `Config SQLx (Rust)`
  - **Prompt:** "Tabelas trades, positions PK (user_id, asset_id)."

- [ ] `interface:controller` /api/trades/* (~2h)
  - **Agent:** `Backend Controller (Rust)`
  - **Prompt:** "POST buy/sell, GET trades, GET :id. JWT owner."

- [ ] `test:unit` Trading + calculator (~3h)
  - **Agent:** `Unit Tests (Rust)`
  - **Prompt:** "Testes SellValidator, PortfolioCalculator, BuyAsset, SellAsset. ≥95%."

- [ ] `test:e2e` Fluxo buy → sell (~2h)
  - **Agent:** `E2E Tests (Rust)`
  - **Prompt:** "E2E: login, buy, verify position, sell parcial, sell excesso falha."

### US-006: Telas de negociação (Leptos)

> Como investidor, quero telas para comprar, vender e ver histórico, para operar a carteira pela interface web.

**Prioridade**: Must
**Estimativa**: 8 pontos
**Ref**: RF-011, RF-012, RF-013

**Critérios de Aceitação**:

- [ ] Dado `/trades/buy`, quando seleciona ativo ativo e confirma, então operação registrada
- [ ] Dado `/trades/sell`, quando qty > posição, então erro exibido com saldo disponível
- [ ] Dado `/trades`, quando carrega, então lista BUY/SELL com datas e totais

### Telas e fluxos (web)

| Rota / tela | Persona | Ação | API | Form? |
|-------------|---------|------|-----|-------|
| `/trades` | investidor | histórico | GET `/api/trades` | Não |
| `/trades/buy` | investidor | comprar | POST `/api/trades/buy` | Sim |
| `/trades/sell` | investidor | vender | POST `/api/trades/sell` | Sim |

**Navegação**: menu "Operações"; venda pode iniciar de detalhe de posição (EP-004).

**Tasks**:

- [ ] `interface:entity` TradeListItem, BuyTradeForm, SellTradeForm (~1h)
  - **Agent:** `Frontend Entity (Leptos)`
  - **Prompt:** "Entidades UI para trades e forms buy/sell."

- [ ] `interface:usecase` List, Buy, Sell, LoadActiveAssets (~2h)
  - **Agent:** `Frontend UseCase (Leptos)`
  - **Prompt:** "Use cases trading + dropdown ativos ativos."

- [ ] `interface:repository` TradeHttpRepository (~2h)
  - **Agent:** `Frontend Repository (Leptos)`
  - **Prompt:** "HTTP buy/sell/list trades."

- [ ] `interface:page` Histórico trades (~2h)
  - **Agent:** `Frontend Page (Leptos)`
  - **Prompt:** "Página /trades paginada."

- [ ] `interface:form-web` Forms buy e sell (~2h)
  - **Agent:** `Frontend Form (Leptos)`
  - **Prompt:** "Forms /trades/buy e /trades/sell; maxQuantity no sell."

- [ ] `test:unit-web` Trading UI use cases (~1h)
  - **Agent:** `Frontend UseCase (Leptos)`
  - **Prompt:** "Testes BuyAssetUseCase/SellAssetUseCase mock."

- [ ] `quality:ci-verify` + `quality:memory-leak` (~1h)
  - **Agent:** `Config CI/CD (Rust)`
  - **Prompt:** "CI verde EP-003; memory check após E2E trading."

---

## EP-004: Portfolio — Carteira

**Bounded Context**: BC Portfolio
**Descrição**: Consulta de posições consolidadas e detalhe por ativo
**Tamanho**: M
**Dependências**: EP-003
**Release**: 1
**OpenSpec**: `ep-004-portfolio`

### US-007: API de carteira

> Como investidor, quero consultar minha carteira e posições via API, para acompanhar quantidades e preço médio.

**Prioridade**: Must
**Estimativa**: 5 pontos
**Ref**: RF-009, RF-010, RN-007, RN-008

**Critérios de Aceitação**:

- [ ] Dado trades existentes, quando GET portfolio, então posições com qty e averagePrice corretos
- [ ] Dado posição inexistente, quando GET portfolio/assets/:id, então 404 ou qty zero
- [ ] Dado JWT, quando GET portfolio, então apenas carteira do owner

**Tasks**:

- [ ] `domain:entity` Portfolio, Position (~2h)
  - **Agent:** `Core Entity (Rust)`
  - **Prompt:** "Portfolio AR com Position filha; invariante qty ≥ 0."

- [ ] `app:query` GetUserPortfolio, GetPositionByAsset (~2h)
  - **Agent:** `Core Query CQRS (Rust)`
  - **Prompt:** "Queries enriquecem ticker/nome do asset (join/conformist)."

- [ ] `app:dto` PortfolioOut, PositionOut (~1h)
  - **Agent:** `Core DTO (Rust)`
  - **Prompt:** "DTOs carteira e posição."

- [ ] `interface:controller` GET /api/portfolio (~1h)
  - **Agent:** `Backend Controller (Rust)`
  - **Prompt:** "GET /api/portfolio e /api/portfolio/assets/:assetId."

- [ ] `test:unit` Queries portfolio (~2h)
  - **Agent:** `Unit Tests (Rust)`
  - **Prompt:** "Testes queries com positions materializadas mock."

- [ ] `test:e2e` E2E portfolio após trades (~2h)
  - **Agent:** `E2E Tests (Rust)`
  - **Prompt:** "E2E: buy → GET portfolio reflete posição."

### US-008: Telas de carteira (Leptos)

> Como investidor, quero ver minha carteira e detalhe de posições, para decidir vendas parciais.

**Prioridade**: Must
**Estimativa**: 5 pontos
**Ref**: RF-009, RF-010

**Critérios de Aceitação**:

- [ ] Dado `/portfolio`, quando carrega, então exibe posições ou empty state
- [ ] Dado detalhe posição, quando clica "Vender", então navega `/trades/sell` com asset pré-selecionado

### Telas e fluxos (web)

| Rota / tela | Persona | Ação | API | Form? |
|-------------|---------|------|-----|-------|
| `/portfolio` | investidor | ver carteira | GET `/api/portfolio` | Não |
| `/portfolio/assets/:id` | investidor | detalhe posição | GET `/api/portfolio/assets/:id` | Não |

**Tasks**:

- [ ] `interface:entity` PortfolioSummary, PositionDetail (~1h)
  - **Agent:** `Frontend Entity (Leptos)`
  - **Prompt:** "Entidades UI carteira e posição."

- [ ] `interface:usecase` LoadPortfolio, LoadPosition (~1h)
  - **Agent:** `Frontend UseCase (Leptos)`
  - **Prompt:** "Use cases leitura carteira."

- [ ] `interface:repository` PortfolioHttpRepository (~2h)
  - **Agent:** `Frontend Repository (Leptos)`
  - **Prompt:** "HTTP GET portfolio endpoints."

- [ ] `interface:page` Carteira e detalhe (~2h)
  - **Agent:** `Frontend Page (Leptos)`
  - **Prompt:** "Páginas /portfolio e /portfolio/assets/:id; link vender."

- [ ] `test:unit-web` Portfolio UI (~1h)
  - **Agent:** `Frontend UseCase (Leptos)`
  - **Prompt:** "Testes LoadPortfolioUseCase mock."

- [ ] `quality:ci-verify` + `quality:memory-leak` Fechamento MVP (~1h)
  - **Agent:** `Config CI/CD (Rust)`
  - **Prompt:** "CI verde release 1 completa; memory check final."

---

## Rastreabilidade requisitos → épicos

| Requisito | Épico |
|-----------|-------|
| RF-001–004, RN-001–003 | EP-001 |
| RF-005–008, RN-004–006 | EP-002 |
| RF-011–014, RN-010–012 | EP-003 |
| RF-009–010, RN-007–009 | EP-004 |
| RNF-001–007 | EP-000 + transversal |

## Lacunas / suposições

- `[suposição]` Seed de usuário Admin inicial no bootstrap ou migration
- `[suposição]` Frontend Leptos no MVP Release 1 (backend pode ser entregue antes)
- `[suposição]` Preços informados manualmente na compra/venda (sem cotação externa)
