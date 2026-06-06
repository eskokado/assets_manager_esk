## Context

**Assets Manage** possui workspace Rust (EP-000): `shared-kernel`, `api` (Axum + sqlx), `web-leptos` (SSR), Postgres e CI. Não há autenticação — qualquer rota futura de carteira ou catálogo ficaria exposta. O backlog EP-001 define auth manual no módulo `auth` (sem pacote auth core separado), JWT + bcrypt, roles Admin/Investor e telas Leptos.

**Referências**: `docs/planning/assets-manage/backlog.md` (US-001, US-002), `ddd-tactical-model.md` BC Auth, `delivery-profile.md` (Leptos, sem mobile).

## Goals / Non-Goals

**Goals:**

- Módulo `auth` completo inside-out: VOs → User → use cases → sqlx → rotas HTTP
- Registro cria Investor; login emite JWT com userId e role; GET me retorna perfil sem hash
- Middleware JWT reutilizável por EP-002+ (Assets admin, Trading investor)
- Telas `/login`, `/register`, `/profile` com guard e erros inline
- Cobertura ≥95% domain+application do módulo auth; E2E HTTP register → login → me
- CI verde incluindo auth backend + Leptos auth

**Non-Goals:**

- OAuth, refresh tokens, recuperação de senha, MFA
- RBAC granular com permissions (apenas enum Role Admin/Investor)
- Mobile
- Seed massivo de usuários — apenas Admin opcional para dev

## Decisions

### 1. Auth no módulo Axum (não crate separado)

**Decisão**: Bounded context `modules/auth` com camadas domain/application/infrastructure/interfaces.

**Rationale**: `delivery-profile.md` — "Auth MVP manual (sem Config Auth Core Rust)". Alinhado a `config-new-module-rs`.

**Alternativa rejeitada**: Crate `auth-core` workspace — overhead sem reuso cross-app no MVP.

### 2. JWT stateless + bcrypt

**Decisão**: `jsonwebtoken` com secret em `JWT_SECRET`; senhas com bcrypt via crate `bcrypt`.

**Rationale**: RF-002/003 e RN-003; padrão inferido em requirements.md.

**Alternativa rejeitada**: Sessions server-side — mais estado e complexidade de deploy.

### 3. VOs auth no módulo (Email reutiliza shared-kernel)

**Decisão**: `Email` do `shared-kernel`; `Password`, `HashPassword`, `UserName`, `Role` em `auth/domain`.

**Rationale**: Email já existe no kernel; Password/HashPassword são específicos de auth.

### 4. Middleware Axum extractable

**Decisão**: Extension `AuthUser { user_id, role }` após validar Bearer token; rotas protegidas usam extractor.

**Rationale**: EP-002 Assets usará mesmo middleware para guard Admin.

### 5. Sessão web Leptos

**Decisão**: Token JWT em cookie httpOnly (preferido) ou localStorage com guard client-side `[inferido: cookie httpOnly se SSR permitir]`.

**Rationale**: ddd-tactical-model — storage seguro; evitar XSS com httpOnly quando possível.

**Alternativa rejeitada**: Token só em memory sem persistência — UX ruim em refresh.

### 6. Estrutura web Leptos auth

**Decisão**: `AuthSession`, `UserProfile` + `IAuthRepository` (reqwest) + use cases UI; páginas SSR com signals.

**Rationale**: Skills `frontend-entity-leptos` → `frontend-page-leptos`; ordem inside-out no frontend.

### 7. Seed Admin

**Decisão**: Migration ou seed SQL com um usuário Admin (env ou fixture dev) para testar EP-002.

**Rationale**: Open question do bootstrap; necessário para demo admin sem registro manual.

## Risks / Trade-offs

| Risco | Mitigação |
|-------|-----------|
| JWT secret fraco em dev | `.env.example` documenta secret longo; CI usa secret de teste |
| Cookie httpOnly + SSR Leptos complexo | Fallback localStorage documentado; testes E2E cobrem fluxo escolhido |
| Duplicação Email VO kernel vs auth | Auth importa `shared_kernel::Email` exclusivamente |
| Cobertura 95% auth atrasa entrega | Gate CI expande para módulo auth; mocks de UserRepository |

## Migration Plan

1. Aplicar migration `users` em Postgres (dev/CI)
2. Deploy API com rotas auth + `JWT_SECRET` configurado
3. Deploy web-leptos com rotas `/login`, `/register`, `/profile`
4. Rotas existentes do shell permanecem; privadas passam a exigir auth após EP-001

**Rollback**: Reverter migration users (se sem dados downstream); desmontar rotas auth; shell volta a dashboard público.

## Open Questions

- Cookie httpOnly vs localStorage para JWT no Leptos — validar na implementação US-002
- Admin seed: migration fixa vs script `sqlx` seed — preferir seed documentado em README dev
