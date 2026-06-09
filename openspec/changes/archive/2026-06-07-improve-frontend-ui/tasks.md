## 1. Infraestrutura e Seeding

- [x] 1.1 Criar migration de seed para o Dashboard. **Agent**: `config-sqlx-rs`. **Prompt**: Criar uma nova migration SQL em `migrations/` que popule as tabelas `users`, `asset_categories`, `assets`, `portfolios` e `transactions` com dados realistas para demonstração do dashboard de gestão de ativos.

## 2. Layout do Frontend (Admin Shell)

- [x] 2.1 Refatorar estado do menu lateral. **Agent**: `frontend-page-leptos`. **Prompt**: No arquivo `crates/web-leptos/src/layouts/admin_shell.rs`, mover o sinal `sidebar_open` para um contexto global ou garantir que ele possa ser acessado pelo componente de Header para permitir o toggle.
- [x] 2.2 Atualizar o cabeçalho com toggle e info de usuário. **Agent**: `frontend-page-leptos`. **Prompt**: No `AdminShell` (em `crates/web-leptos/src/layouts/admin_shell.rs`), atualizar o `<header>` para: 1. Incluir o botão de toggle do menu no lado esquerdo (visível em desktop também); 2. Exibir o nome e e-mail do usuário logado no lado direito (consumindo do auth context); 3. Incluir um link/botão para visualizar o perfil (`/profile`).
- [x] 2.3 Incluir ícones e comportamento colapsável no menu. **Agent**: `frontend-page-leptos`. **Prompt**: No componente `SidebarMenu` (`crates/web-leptos/src/components/sidebar_menu.rs`), adicionar ícones SVG para cada item de menu. Garantir que quando `open` for falso, a sidebar encolha exibindo apenas os ícones.
- [x] 2.4 Adicionar botão de logout no menu. **Agent**: `frontend-page-leptos`. **Prompt**: No `SidebarMenu`, adicionar um botão "Sair" no final da lista ou fixo no bottom da sidebar que execute o fluxo de logout chamando o `AuthUseCase` correspondente.

## 3. Dashboard Profissional

- [x] 3.1 Implementar KPIs no Dashboard. **Agent**: `frontend-page-leptos`. **Prompt**: Na `DashboardPage` (`crates/web-leptos/src/pages/dashboard.rs`), criar um grid de cartões (KPIs) exibindo "Saldo Total", "Rentabilidade", "Ativos Ativos" e "Dividendos Previstos", utilizando classes Tailwind para um visual premium (glassmorphism/gradients).
- [x] 3.2 Implementar lista de transações recentes. **Agent**: `frontend-page-leptos`. **Prompt**: Adicionar à `DashboardPage` uma seção de "Últimas Transações" exibindo uma tabela ou lista estilizada com as movimentações recentes recuperadas via UseCase/Repository.
- [x] 3.3 Adicionar visualização de alocação. **Agent**: `frontend-page-leptos`. **Prompt**: Incluir um componente visual simples (como barras de progresso ou um mini-gráfico de pizza se houver biblioteca disponível) mostrando a alocação por classe de ativos no dashboard.
