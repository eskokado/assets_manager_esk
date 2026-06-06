# assets_manage

Carteira de investimentos — Rust (Axum + sqlx) + Leptos SSR.

## Stack

- **API**: `crates/api` (Axum, PostgreSQL/sqlx)
- **Shared kernel**: `crates/shared-kernel`
- **Web**: `crates/web-leptos` (Leptos SSR + Tailwind v4)

## Ambientes

| | Desenvolvimento | Produção |
|---|-----------------|----------|
| **Compose** | `docker-compose.yml` | `docker-compose.prod.yml` |
| **O que sobe no Docker** | Somente **Postgres** | **Postgres** + **API** |
| **API** | Host — `cargo run -p api` (`:4000`) | Container — imagem do `Dockerfile` |
| **Web (Leptos)** | Host — `cargo leptos watch` (`:3000`) | *(web em prod: ver nota abaixo)* |
| **Hot reload** | Sim (cargo / leptos watch) | Não — rebuild de imagem |

**Por que dev no host?** Compilação incremental, `cargo leptos watch` e sqlx apontando para `localhost:5432` — padrão definido no EP-000 e mantido nos épicos de negócio (EP-001+).

> **Nota prod:** o `docker-compose.prod.yml` atual sobe API + Postgres. Imagem do frontend Leptos pode ser adicionada quando fechar o deploy full-stack.

## Pré-requisitos

Ferramentas para desenvolvimento local (API + Leptos no host, Postgres no Docker):

| Ferramenta | Versão mínima | Uso |
|------------|---------------|-----|
| [Rust](https://rustup.rs/) (stable) | 1.75+ | API, shared-kernel, Leptos SSR |
| [Docker](https://docs.docker.com/get-docker/) + Compose | v2 | Postgres em dev/prod |
| [Node.js](https://nodejs.org/) | 18+ | Tailwind CSS v4 (`npm run watch:css`) |
| **cargo-leptos** | 0.2.x | CLI `cargo leptos watch` (não vem com o Rust) |
| **wasm32-unknown-unknown** | — | target Rust exigido pelo cargo-leptos |

### Setup inicial (primeira vez)

**1. Rust**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version   # ex.: 1.75+
```

**2. Dependências de sistema (Linux / WSL)**

Necessárias para compilar crates nativos (`openssl`, `libgit2`, etc.):

```bash
# Debian / Ubuntu / WSL
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev

# Fedora
sudo dnf groupinstall "Development Tools"
sudo dnf install openssl-devel pkg-config
```

**3. Target WebAssembly + cargo-leptos**

O Leptos **0.7.8** deste projeto usa **cargo-leptos 0.2.x** (não confundir com 0.3.x, voltado ao Leptos 0.8):

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --locked --version 0.2.42
cargo leptos --version   # ex.: cargo-leptos 0.2.42
```

> A instalação do `cargo-leptos` compila dezenas de crates e pode levar **10–20 min** na primeira vez. O binário fica em `~/.cargo/bin/` — confirme que esse diretório está no `PATH`.

**4. Node.js (Tailwind)**

```bash
npm install   # @tailwindcss/cli + tailwindcss (devDependencies)
```

**5. Variáveis de ambiente**

```bash
cp .env.example .env
```

### Verificação dos pré-requisitos

```bash
rustc --version
cargo leptos --version    # se falhar: "no such command: leptos" → passo 3 acima
docker compose version
node --version
npm run build:css         # gera crates/web-leptos/style/output.css
```

### Problemas comuns

| Erro | Causa | Solução |
|------|-------|---------|
| `error: no such command: leptos` | `cargo-leptos` não instalado | `cargo install cargo-leptos --locked --version 0.2.42` |
| `Please define leptos projects in ... metadata.leptos` | falta config do cargo-leptos | ver `[package.metadata.leptos]` em `crates/web-leptos/Cargo.toml` |
| Incompatibilidade `wasm-bindgen` (0.2.100 vs 0.2.122) | cargo-leptos 0.2.x usa bindgen 0.2.100 | projeto já fixa `wasm-bindgen = "=0.2.100"` e `web-sys = "=0.3.77"` |
| Falha ao compilar `openssl-sys` | headers OpenSSL ausentes | `sudo apt install libssl-dev pkg-config` (Debian/Ubuntu) |
| `wasm32-unknown-unknown` não encontrado | target WASM não adicionado | `rustup target add wasm32-unknown-unknown` |
| CSS sem estilo | Tailwind não buildou | `npm install && npm run build:css` (ou `npm run watch:css`) |

## Desenvolvimento (recomendado)

Após o [setup inicial](#setup-inicial-primeira-vez):

```bash
cp .env.example .env   # se ainda não fez
docker compose up -d          # 1) só Postgres em :5432
```

**Terminal 1 — API**

```bash
cargo run -p api              # :4000 — migrations aplicam no startup
```

**Terminal 2 — CSS (opcional se já rodou `npm run build:css`)**

```bash
npm run watch:css
```

**Terminal 3 — Web**

```bash
npm run build:css             # primeira vez (ou use watch:css no terminal 2)
cargo leptos watch            # :3000 — config em crates/web-leptos/Cargo.toml [package.metadata.leptos]
```

### Verificação rápida

| URL | Esperado |
|-----|----------|
| `GET http://localhost:4000/health` | `{"status":"ok"}` |
| `http://localhost:3000/login` | Tela de login |
| `http://localhost:3000/` | Dashboard (exige login) |

### Auth (EP-001)

- **API:** `POST /api/auth/register`, `POST /api/auth/login`, `GET /api/auth/me` (Bearer JWT)
- **Web:** `/login`, `/register`, `/profile`
- **Seed dev:** `admin@assets.local` / `Senha@12345678` (migration `users`)

### Testes

```bash
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

Requer Postgres acessível (`DATABASE_URL` no `.env`).

## Produção

Build da API e stack com Docker Compose de produção:

```bash
docker compose -f docker-compose.prod.yml up -d --build
```

Variáveis: ajuste `DATABASE_URL`, `JWT_SECRET` e `BIND_ADDR` no compose ou via env antes do deploy.

## Docs

- Discovery: `docs/discovery/assets-manage/`
- Planning: `docs/planning/assets-manage/`
- OpenSpec (ativo): `openspec/changes/`
- OpenSpec (arquivado): `openspec/changes/archive/`
