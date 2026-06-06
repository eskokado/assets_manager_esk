# Requisitos — Assets Manage (Carteira de Investimentos)

**Fonte**: Descrição funcional do projeto (greenfield — sem sistema legado)
**Data da análise**: 2026-06-06
**Método**: Especificação textual + inferência para MVP Rust

## Visão Geral

O **Assets Manage** é um projeto prático de carteira de investimentos com backend em **Rust**, focado em organização de dados, regras de negócio e boas práticas de **performance** e **segurança**. O público-alvo são investidores pessoa física ou equipes internas que precisam registrar ativos financeiros, controlar posições e executar operações de compra e venda de forma auditável.

O MVP cobre quatro pilares: **autenticação e controle de acesso** ao gerenciamento de ativos, **cadastro de ativos** (catálogo), **carteira por usuário** (posições derivadas das operações) e **compra/venda** de ativos para usuários cadastrados. Funcionalidades “inteligentes” (recomendações, scoring, ML) ficam fora do MVP e são registradas como evolução futura `[inferido]`.

## Stack Tecnológica

| Camada         | Tecnologia                                      |
| -------------- | ----------------------------------------------- |
| Backend        | Rust — Axum, Tokio                            |
| Persistência   | PostgreSQL — sqlx, migrations Flyway/SQL `[inferido]` |
| Shared Kernel  | `shared-kernel` (Entity, Result, UseCase)       |
| Autenticação   | JWT + bcrypt `[inferido]`                       |
| Frontend MVP   | API REST (+ painel web admin `[inferido]`)      |
| Banco de Dados | PostgreSQL                                      |
| Infra          | Docker Compose (dev), CI Rust `[inferido]`      |

## Módulos Identificados

### Auth — Gerenciamento de Acesso

**Descrição**: Identidade, autenticação e autorização para proteger o gerenciamento de ativos. Apenas usuários autenticados acessam carteira e operações; perfil administrativo gerencia catálogo de ativos.

#### Requisitos Funcionais

- **RF-001**: Registrar usuário
  - Endpoint: `POST /api/auth/register` `[inferido]`
  - Campos: nome, e-mail, senha
  - Validações: e-mail válido, senha mínima, e-mail único
  - Ações: cria usuário com perfil padrão (investidor)

- **RF-002**: Autenticar usuário (login)
  - Endpoint: `POST /api/auth/login` `[inferido]`
  - Campos: e-mail, senha
  - Validações: credenciais corretas
  - Ações: retorna token JWT com claims (userId, role)

- **RF-003**: Consultar perfil autenticado
  - Endpoint: `GET /api/auth/me` `[inferido]`
  - Validações: token JWT válido
  - Ações: retorna dados do usuário logado

- **RF-004**: Controle de acesso por perfil (MVP)
  - Perfis: **Admin** (gerencia ativos), **Investidor** (opera carteira própria)
  - Validações: middleware/guard por rota
  - Ações: bloqueia operações não autorizadas (403)

#### Regras de Negócio

- **RN-001**: Apenas **Admin** pode criar, editar ou desativar ativos no catálogo.
- **RN-002**: **Investidor** só visualiza e opera a **própria** carteira.
- **RN-003**: Senha armazenada apenas como hash (bcrypt/argon2) — nunca em texto plano.

---

### Assets — Catálogo de Ativos

**Descrição**: Registro mestre dos instrumentos financeiros negociáveis na plataforma (ações, FIIs, ETFs, etc.).

#### Requisitos Funcionais

- **RF-005**: Cadastrar ativo (Admin)
  - Endpoint: `POST /api/assets` `[inferido]`
  - Campos: ticker/símbolo, nome, tipo (enum), moeda, status ativo
  - Validações: ticker único, campos obrigatórios, tipo válido
  - Ações: persiste ativo disponível para negociação

- **RF-006**: Listar ativos
  - Endpoint: `GET /api/assets` `[inferido]`
  - Filtros: tipo, status, busca por ticker/nome `[inferido]`
  - Ações: retorna lista paginada

- **RF-007**: Buscar ativo por ID
  - Endpoint: `GET /api/assets/:id` `[inferido]`
  - Ações: retorna detalhe ou 404

- **RF-008**: Atualizar ativo (Admin)
  - Endpoint: `PUT /api/assets/:id` `[inferido]`
  - Campos: nome, tipo, status (desativar)
  - Validações: não permitir desativar se houver posição aberta `[inferido]`

#### Regras de Negócio

- **RN-004**: Ticker/símbolo é identificador de negócio único no catálogo.
- **RN-005**: Ativo **inativo** não pode ser alvo de novas compras.
- **RN-006**: Tipo de ativo restringe metadados futuros (extensível no pós-MVP).

---

### Portfolio — Carteira do Usuário

**Descrição**: Consolidação das posições do investidor a partir das operações de compra e venda.

#### Requisitos Funcionais

- **RF-009**: Visualizar carteira do usuário autenticado
  - Endpoint: `GET /api/portfolio` `[inferido]`
  - Campos de saída: ativo, quantidade, preço médio, valor investido `[inferido]`
  - Ações: agrega transações confirmadas

- **RF-010**: Detalhar posição em um ativo
  - Endpoint: `GET /api/portfolio/assets/:assetId` `[inferido]`
  - Ações: quantidade atual, histórico resumido de movimentações `[inferido]`

#### Regras de Negócio

- **RN-007**: Quantidade em carteira = soma(compras) − soma(vendas) por ativo e usuário.
- **RN-008**: Preço médio ponderado recalculado a cada compra `[inferido]`.
- **RN-009**: Não é permitido saldo negativo de quantidade (venda bloqueada se insuficiente).

---

### Trading — Compra e Venda de Ativos

**Descrição**: Registro de ordens/operações de compra e venda vinculadas ao usuário autenticado e ao catálogo de ativos.

#### Requisitos Funcionais

- **RF-011**: Registrar compra de ativo
  - Endpoint: `POST /api/trades/buy` `[inferido]`
  - Campos: assetId, quantidade, preço unitário, data da operação `[inferido]`
  - Validações: quantidade > 0, preço > 0, ativo ativo, usuário autenticado
  - Ações: cria transação BUY; atualiza posição na carteira

- **RF-012**: Registrar venda de ativo
  - Endpoint: `POST /api/trades/sell` `[inferido]`
  - Campos: assetId, quantidade, preço unitário, data da operação `[inferido]`
  - Validações: quantidade > 0, posição suficiente, ativo existente
  - Ações: cria transação SELL; reduz posição na carteira

- **RF-013**: Listar transações do usuário
  - Endpoint: `GET /api/trades` `[inferido]`
  - Filtros: tipo (buy/sell), ativo, período `[inferido]`
  - Ações: histórico paginado

- **RF-014**: Consultar transação por ID
  - Endpoint: `GET /api/trades/:id` `[inferido]`
  - Validações: transação pertence ao usuário (ou Admin auditando) `[inferido]`

#### Regras de Negócio

- **RN-010**: Toda operação gera registro imutável (auditoria); correções via estorno `[inferido]` fora do MVP.
- **RN-011**: Valor total da operação = quantidade × preço unitário (Money VO).
- **RN-012**: Venda rejeitada se quantidade solicitada > quantidade em carteira.

---

## Requisitos Não-Funcionais

- **RNF-001**: API REST com latência adequada para operações síncronas de carteira (sem batch pesado no MVP).
- **RNF-002**: Autenticação JWT stateless; secrets via variáveis de ambiente.
- **RNF-003**: Validação de entrada na borda (DTO) e no domínio (VOs com `Result`).
- **RNF-004**: Cobertura de testes unitários ≥ 95% em domain/application `[inferido — padrão do repositório]`.
- **RNF-005**: Migrations versionadas; schema explícito (sqlx).
- **RNF-006**: Logs estruturados sem expor dados sensíveis (senha, token completo).
- **RNF-007**: Concorrência: transações de carteira devem ser consistentes (transação DB por operação buy/sell) `[inferido]`.

## Integrações

| Sistema              | Tipo        | Descrição                                      |
| -------------------- | ----------- | ---------------------------------------------- |
| PostgreSQL           | Banco       | Persistência principal                         |
| Cotações externas    | API REST    | **Fora do MVP** — preços informados manualmente |
| Provedor OAuth       | OAuth 2.0   | **Fora do MVP** — login e-mail/senha primeiro  |

## Perfis de Acesso

| Perfil     | Permissões                                                                 |
| ---------- | -------------------------------------------------------------------------- |
| Admin      | CRUD ativos; listar usuários `[inferido]`; auditar transações globais      |
| Investidor | Ver catálogo; ver própria carteira; comprar/vender; ver próprio histórico  |

## Fluxos Principais

### Fluxo 1: Onboarding e primeira compra

```
Register → Login → Listar ativos → Comprar ativo → Ver carteira atualizada
```

Usuário se cadastra, autentica-se, consulta o catálogo de ativos ativos, registra uma compra informando quantidade e preço, e visualiza a posição consolidada na carteira.

### Fluxo 2: Venda parcial

```
Login → Ver carteira → Vender ativo (qty ≤ posição) → Ver carteira e histórico
```

Investidor consulta posição, registra venda dentro do saldo disponível; sistema rejeita venda excedente.

### Fluxo 3: Administração de catálogo

```
Login (Admin) → Criar/editar ativo → Investidor negocia apenas ativos ativos
```

Administrador mantém o catálogo; investidores só operam instrumentos válidos e ativos.

## Lacunas e Observações

- **Sem sistema legado**: requisitos derivados da descrição do projeto; itens marcados `[inferido]` devem ser validados com stakeholders.
- **“Aplicação inteligente”**: no MVP não há recomendação, alocação automática ou integração com mercado — apenas gestão estruturada de dados e operações.
- **Moeda**: assumir BRL como padrão `[inferido]`; multi-moeda é evolução.
- **Tipos de ativo**: enum inicial (ex.: STOCK, FII, ETF, BOND) `[inferido]`.
- **Frontend**: MVP pode ser API-only; painel web admin/investidor recomendado para demo `[inferido]`.
- **Estorno/cancelamento** de operação não especificado — fora do MVP.
