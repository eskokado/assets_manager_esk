# Tasks — ep-001-auth (EP-001)

**Stories**: US-001 API de autenticação · US-002 Telas de autenticação (Leptos)  
**Stack**: Rust (Axum + sqlx) · Leptos SSR · Postgres  
**Ref**: `docs/planning/assets-manage/backlog.md` EP-001

## 1. Domínio — Value Objects e entidade

- [x] 1.1 `domain:vo` VOs Email, Password, HashPassword, UserName, Role (~2h)
  - **Agent:** `Core Value Object (Rust)`
  - **Prompt:** Crie VOs Password, HashPassword, UserName, Role em modules/auth/domain. Reutilize Email do shared-kernel. Create() retorna Result. Password min 8 chars.

- [x] 1.2 `domain:entity` Entidade User (~2h)
  - **Agent:** `Core Entity (Rust)`
  - **Prompt:** Aggregate User com register() e verify_password(). Role default Investor. modules/auth/domain/entity.

- [x] 1.3 `domain:service` PasswordPolicy (~1h)
  - **Agent:** `Core Domain Service (Rust)`
  - **Prompt:** PasswordPolicy valida força da senha no registro (min 8 chars). modules/auth/domain/services.

- [x] 1.4 `domain:repository` Port UserRepository (~1h)
  - **Agent:** `Core Repository (Rust)`
  - **Prompt:** Trait UserRepository: save, find_by_id, find_by_email, exists_by_email. modules/auth/domain/ports.

## 2. Application — DTOs, use cases e query

- [x] 2.1 `app:dto` RegisterIn, LoginIn, AuthTokenOut, UserOut (~1h)
  - **Agent:** `Core DTO (Rust)`
  - **Prompt:** DTOs serde para register, login, token e perfil. modules/auth/application/dto.

- [x] 2.2 `app:usecase` RegisterUser, LoginUser (~2h)
  - **Agent:** `Core Use Case (Rust)`
  - **Prompt:** RegisterUser verifica e-mail único, hash bcrypt, persiste Investor. LoginUser valida credenciais, emite JWT com userId e role.

- [x] 2.3 `app:query` GetCurrentUser (~1h)
  - **Agent:** `Core Query CQRS (Rust)`
  - **Prompt:** GetCurrentUser por userId do JWT retorna UserOut sem password_hash.

## 3. Infraestrutura — persistência

- [x] 3.1 `infra:migration` Migration users (~1h)
  - **Agent:** `Config SQLx (Rust)`
  - **Prompt:** Migration users: id, name, email unique, password_hash, role, active, created_at. Seed Admin opcional para dev.

- [x] 3.2 `infra:persistence` UserSqlxRepository (~2h)
  - **Agent:** `Backend Data (Rust)`
  - **Prompt:** Adapter sqlx para users; mapeamento UserRecord ↔ User. Implementa UserRepository.

## 4. API — rotas e middleware JWT

- [x] 4.1 `interface:controller` Rotas auth (~2h)
  - **Agent:** `Backend Controller (Rust)`
  - **Prompt:** POST /api/auth/register, POST /api/auth/login, GET /api/auth/me. Middleware JWT reutilizável com AuthUser extractor. JWT_SECRET em .env.example.

## 5. Testes backend

- [x] 5.1 `test:unit` Testes domain + use cases (~2h)
  - **Agent:** `Unit Tests (Rust)`
  - **Prompt:** Testes VOs, User, PasswordPolicy, RegisterUser, LoginUser, GetCurrentUser com mocks. Cobertura ≥95% domain+application do módulo auth.

- [x] 5.2 `test:e2e` E2E register → login → me (~2h)
  - **Agent:** `E2E Tests (Rust)`
  - **Prompt:** Fluxo HTTP: POST register (201), POST login (JWT), GET /api/auth/me com Bearer token (200). Credenciais inválidas retornam 401.

## 6. Web — entidades, use cases e repository

- [x] 6.1 `interface:entity` AuthSession, UserProfile (~1h)
  - **Agent:** `Frontend Entity (Leptos)`
  - **Prompt:** Entidades AuthSession (accessToken, expiresIn, userId, role) e UserProfile (id, name, email, role) com shared_kernel::Result, sem leptos::* no domain. crate web-leptos.

- [x] 6.2 `interface:usecase` Login, Register, LoadProfile (~2h)
  - **Agent:** `Frontend UseCase (Leptos)`
  - **Prompt:** LoginUseCase, RegisterUseCase, LoadProfileUseCase async orquestrando IAuthRepository; retorno Result. Login persiste sessão; Register redireciona para login.

- [x] 6.3 `interface:repository` AuthHttpRepository (~2h)
  - **Agent:** `Frontend Repository (Leptos)`
  - **Prompt:** reqwest adapter: POST login, POST register, GET me. Mapear DTOs API → AuthSession/UserProfile. Base URL via config/proxy.

## 7. Web — formulários e páginas

- [x] 7.1 `interface:form-web` Formulários login e register (~2h)
  - **Agent:** `Frontend Form (Leptos)`
  - **Prompt:** Forms SSR /login e /register com signals; submit chama use cases; erros 400/422 inline no formulário.

- [x] 7.2 `interface:page` Páginas login, register, profile (~2h)
  - **Agent:** `Frontend Page (Leptos)`
  - **Prompt:** Rotas /login, /register, /profile. Guard: rotas privadas redirect /login sem token. /profile exibe nome, email, role via LoadProfileUseCase. Login success → dashboard/carteira.

- [x] 7.3 `infra:shell-web` Integrar auth no shell (~1h)
  - **Agent:** `Config Shared Web (Leptos)`
  - **Prompt:** Menu "Perfil" visível se autenticado. Rotas públicas /login, /register fora do guard privado. Private routes redirect /login.

## 8. Testes web e fechamento

- [x] 8.1 `test:unit-web` Testes use cases auth UI (~1h)
  - **Agent:** `Frontend UseCase (Leptos)`
  - **Prompt:** Testes unitários LoginUseCase, RegisterUseCase, LoadProfileUseCase com mock IAuthRepository.

- [x] 8.2 `quality:ci-verify` CI verde EP-001 (~30min)
  - **Agent:** `Config CI/CD (Rust)`
  - **Prompt:** Garantir CI verde com módulo auth backend + telas Leptos + E2E auth.

- [x] 8.3 `quality:memory-leak` Memory check (~30min)
  - **Agent:** `Config CI/CD (Rust)`
  - **Prompt:** Rodar check-memory-rs.sh após E2E auth; corrigir vazamentos Arc/tasks.

## Critérios de aceitação (US-001)

- [x] Dado e-mail novo, quando POST register, então usuário Investor é criado e retorna 201
- [x] Dado credenciais válidas, quando POST login, então retorna JWT com userId e role
- [x] Dado token válido, quando GET me, então retorna perfil sem passwordHash
- [x] Dado e-mail duplicado, quando register, então retorna erro de negócio
- [x] Dado senha em texto plano, quando persistido, então apenas hash bcrypt é armazenado

## Critérios de aceitação (US-002)

- [x] Dado usuário em `/login`, quando credenciais corretas, então redireciona para dashboard/carteira
- [x] Dado usuário em `/register`, quando formulário válido, então conta criada e redireciona login
- [x] Dado rota privada sem token, quando acessada, então redirect `/login`
- [x] Dado usuário em `/profile`, quando autenticado, então exibe nome, e-mail e role
