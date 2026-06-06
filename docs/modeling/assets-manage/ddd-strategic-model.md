# Modelo Estratégico — Assets Manage

**Baseado em**: `docs/discovery/assets-manage/` (requirements, ddd-analysis, delivery-inventory)
**Data da modelagem**: 2026-06-06

## Domínio

| Aspecto | Descrição |
| ------- | --------- |
| **Nome** | Assets Manage — Carteira de Investimentos |
| **Propósito** | Permitir que investidores registrem e acompanhem posições em ativos financeiros, executando compras e vendas de forma auditável, com catálogo administrado e controle de acesso por perfil. |
| **Público** | Investidores pessoa física; administradores que mantêm o catálogo de instrumentos. |

### Escopo

**Dentro do domínio (MVP):**

- Identidade e autorização (Admin / Investidor)
- Catálogo de ativos financeiros (ticker, tipo, moeda, status)
- Carteira consolidada por usuário (posições, preço médio)
- Operações de compra e venda vinculadas ao usuário autenticado
- API REST Rust (Axum) como superfície principal

**Fora do domínio (MVP):**

- Recomendações inteligentes, scoring, alocação automática
- Cotações em tempo real / integração com mercado
- OAuth, RBAC granular, estorno de operações
- App mobile
- Multi-moeda avançada (além de BRL padrão `[inferido]`)

---

## Subdomínios

| Subdomínio | Tipo | Responsabilidade | Complexidade | Estratégia |
| ---------- | ---- | ---------------- | ------------ | ---------- |
| **Negociação (Trading)** | Core | Registrar compras/vendas, validar saldo, auditar operações | Alta | Build |
| **Carteira (Portfolio)** | Core | Consolidar posições e preço médio a partir das negociações | Alta | Build |
| **Catálogo de Ativos** | Supporting | Manter instrumentos negociáveis (CRUD Admin) | Média | Build |
| **Identidade e Acesso** | Generic | Registro, login JWT, roles Admin/Investidor | Baixa | Build (MVP) / Buy (OAuth pós-MVP) |

```
┌─────────────────────────────────────────────────────────────┐
│              MAPA DE SUBDOMÍNIOS — Assets Manage            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   ┌─────────────────┐         ┌─────────────────┐          │
│   │    CORE         │         │   SUPPORTING    │          │
│   │  Trading        │◀───────▶│  Catálogo       │          │
│   │  Portfolio      │         │  de Ativos      │          │
│   └────────┬────────┘         └────────┬────────┘          │
│            │                           │                    │
│            └───────────┬───────────────┘                    │
│                        ▼                                    │
│              ┌─────────────────┐                            │
│              │    GENERIC      │                            │
│              │  Identidade     │                            │
│              │  e Acesso       │                            │
│              └─────────────────┘                            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Bounded Contexts

| BC | Subdomínio(s) | Cardinalidade | Responsabilidade | Módulo no código |
| -- | ------------- | ------------- | ---------------- | ---------------- |
| **Auth** | Identidade e Acesso | 1:1 | Usuários, credenciais, JWT, roles | `modules/auth` |
| **Assets** | Catálogo de Ativos | 1:1 | Cadastro mestre de instrumentos | `modules/assets` |
| **Portfolio** | Carteira | 1:1 | Posições materializadas por usuário | `modules/portfolio` |
| **Trading** | Negociação | 1:1 | Transações BUY/SELL imutáveis | `modules/trading` |

> Cardinalidade **1:1** adotada como default: cada subdomínio MVP mapeia para um módulo Axum isolado. BC futuro **Insights** (1:N do subdomínio Carteira) fica fora do MVP.

---

## Context Map

```
┌─────────────────────────────────────────────────────────────────────┐
│                         CONTEXT MAP                                  │
│                                                                      │
│  ┌──────────┐                                                        │
│  │   Auth   │════════ upstream (Customer/Supplier) ═══════╗          │
│  │ (Generic)│  fornece UserId, Role via JWT               ║          │
│  └────┬─────┘                                              ║          │
│       │ upstream                                           ║          │
│       ▼                                                    ║          │
│  ┌──────────┐    [OHS]     ┌───────────┐    commands     ║          │
│  │  Assets  │─────────────▶│  Trading  │─────────────────╝          │
│  │(Support.)│  AssetId,    │  (Core)   │                              │
│  │          │  Ticker,     └─────┬─────┘                              │
│  │          │  active flag       │ downstream                         │
│  └────┬─────┘                    │ (sync update)                      │
│       │ shared kernel            ▼                                    │
│       │ (Money, Id)        ┌───────────┐                              │
│       └───────────────────▶│ Portfolio │                              │
│         conformist read    │  (Core)   │                              │
│                            └───────────┘                              │
│                                                                      │
│  Relações:                                                           │
│    Auth ──▶ Trading, Portfolio, Assets (write guard)                 │
│    Assets [OHS] ──▶ Trading (consulta ativo ativo)                   │
│    Trading ──▶ Portfolio (atualização atômica de posição)            │
│    Portfolio ◀── conformist ── Assets (ticker/nome para exibição)    │
│                                                                      │
│  Futuro (Separate Ways): Insights/Advisory ◀── read ── Portfolio     │
└─────────────────────────────────────────────────────────────────────┘
```

### Relações tipadas

| De | Para | Relação | Descrição |
| -- | ---- | ------- | --------- |
| Auth | Trading, Portfolio, Assets | Upstream / Customer-Supplier | JWT identifica usuário; Trading/Portfolio consomem claims |
| Assets | Trading | Open Host Service | Trading consulta existência e flag `active` do ativo |
| Assets | Portfolio | Shared Kernel (parcial) | `AssetId`, `Ticker` como identificadores compartilhados |
| Portfolio | Assets | Conformist | Portfolio exibe ticker/nome sem redefinir modelo de catálogo |
| Trading | Portfolio | Downstream | Buy/Sell dispara recálculo de posição na mesma transação DB |
| Auth | Assets (escrita) | Upstream | Role Admin obrigatória para POST/PUT |

---

## Linguagem Ubíqua

### BC Auth

| Termo | Significado neste contexto |
| ----- | -------------------------- |
| Usuário | Pessoa com conta na plataforma (identidade única por e-mail) |
| Investidor | Usuário com role padrão; opera apenas a própria carteira |
| Administrador | Usuário com permissão de escrita no catálogo de ativos |
| Sessão | Token JWT emitido após login válido |
| Credencial | Par e-mail + senha (senha nunca persistida em plain text) |

### BC Assets

| Termo | Significado neste contexto |
| ----- | -------------------------- |
| Ativo | Instrumento financeiro cadastrado (ação, FII, ETF, título) |
| Ticker | Símbolo único de negociação (ex.: PETR4, HGLG11) |
| Tipo de ativo | Classificação enum (STOCK, FII, ETF, BOND) |
| Ativo ativo | Instrumento elegível para novas compras |
| Catálogo | Conjunto de todos os ativos registrados |

### BC Portfolio

| Termo | Significado neste contexto |
| ----- | -------------------------- |
| Carteira | Visão consolidada das posições de um investidor |
| Posição | Quantidade detida de um ativo específico |
| Preço médio | Custo médio ponderado das compras da posição |
| Saldo | Quantidade disponível para venda (≥ 0) |

> **Nota**: "Ativo" no catálogo descreve o instrumento; na carteira, "posição em ativo" descreve quantidade detida — vocabulário compartilhado com nuance contextual.

### BC Trading

| Termo | Significado neste contexto |
| ----- | -------------------------- |
| Operação / Trade | Registro imutável de compra ou venda |
| Compra (BUY) | Entrada de quantidade na carteira |
| Venda (SELL) | Saída de quantidade; bloqueada se saldo insuficiente |
| Preço unitário | Valor pago/recebido por unidade na operação |
| Valor total | quantidade × preço unitário |
