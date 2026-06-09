## Why

Com o bootstrap (EP-000) concluído, a plataforma **Assets Manage** ainda não protege recursos nem identifica usuários. O épico EP-001 entrega identidade e acesso — registro, login JWT, perfil autenticado e roles Admin/Investor — desbloqueando EP-002 (Assets), EP-003 (Trading) e EP-004 (Portfolio), que dependem de autenticação e autorização por perfil.

## What Changes

- Criar módulo `auth` em `crates/api/src/modules/auth/` com domínio (User, VOs, PasswordPolicy), application (RegisterUser, LoginUser, GetCurrentUser) e infra (UserSqlxRepository)
- Adicionar migration `users` (id, name, email unique, password_hash, role, active, created_at)
- Expor `POST /api/auth/register`, `POST /api/auth/login`, `GET /api/auth/me` com middleware JWT
- Hash bcrypt para senhas; role padrão `Investor` no registro
- Implementar telas Leptos `/login`, `/register`, `/profile` com guard de rotas privadas
- Integrar menu "Perfil" e redirect para `/login` em rotas protegidas no shell web
- Testes unitários (domain + application ≥95%), E2E register → login → me, testes use cases UI e CI verde

## Capabilities

### New Capabilities

- `auth-api`: Endpoints REST de registro, login e perfil; emissão e validação JWT; middleware de autenticação
- `auth-persistence`: Schema `users`, adapter sqlx UserRepository e seed opcional de Admin
- `auth-web`: Telas Leptos login/register/profile, entidades UI, use cases e AuthHttpRepository

### Modified Capabilities

- `web-admin-shell`: Rotas públicas vs privadas, item de menu "Perfil" quando autenticado, redirect `/login` sem token

## Impact

- **Código**: `crates/api/src/modules/auth/**`, migration em `migrations/`, `crates/web-leptos` (rotas auth, guards, forms)
- **APIs**: novos endpoints `/api/auth/*`; rotas futuras protegidas por JWT (EP-002+)
- **Dependências**: `jsonwebtoken`, `bcrypt` (ou equivalente), extensão reqwest no web-leptos
- **Config**: `JWT_SECRET`, `JWT_EXPIRES_IN` em `.env.example`
- **Segurança**: senhas nunca em texto plano; token com claims userId e role
- **Documentação**: alinhado a `docs/planning/assets-manage/backlog.md` EP-001, US-001/US-002 e `ddd-tactical-model.md` BC Auth
