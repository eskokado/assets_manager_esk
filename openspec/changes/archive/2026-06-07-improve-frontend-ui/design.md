## Context

O projeto utiliza **Leptos v0.7 (pre-release/main branch API)** no frontend e uma API Rust no backend. O layout atual (`AdminShell`) possui uma estrutura básica com um `SidebarMenu` e um `header` simples. O dashboard é apenas um placeholder. O sistema de autenticação já está parcialmente implementado, mas as informações do usuário não são refletidas na UI global.

## Goals / Non-Goals

**Goals:**

- Implementar toggle de menu funcional e responsivo.
- Exibir nome e e-mail do usuário logado no header.
- Adicionar ícones e botão de logout ao menu lateral.
- Criar um dashboard visualmente atraente para gestão de ativos.
- Prover dados de teste via seeds.

**Non-Goals:**

- Refatoração completa do motor de autenticação (usaremos o que já existe).
- Implementação de gráficos complexos interativos (usaremos Tailwind/CSS para representações visuais simples ou bibliotecas leves se necessário).

## Decisions

- **Layout State:** O sinal `sidebar_open` será movido para um contexto global ou passado via props para permitir que o botão no Header o controle.
- **Header Enhancement:**
  - Lado esquerdo: Botão ☰ visível em todas as resoluções para colapsar/expandir o menu.
  - Lado direito: Dropdown ou seção estática com `{user.name}` e `{user.email}`, com link para `/profile`.
- **Sidebar Menu:**
  - Adição de ícones SVG (Lucide-like) para cada item de menu.
  - Quando colapsado, o menu exibirá apenas os ícones (tooltip opcional).
  - Footer da sidebar com botão "Sair" integrando com o `AuthUseCase`.
- **Dashboard UI:**
  - Grid de 4 colunas para KPIs (Saldo Total, Lucro/Prejuízo, Ativos Ativos, Dividendos).
  - Lista de "Últimas Movimentações".
  - Componente de "Composição da Carteira" (visual simples com barras de progresso ou mini-gráficos).
- **Data Seeding:**
  - Criação de um novo comando de seed no backend (ou script SQL) para gerar usuários, ativos, categorias e transações mockadas.

## Risks / Trade-offs

- **Leptos 0.7 API:** A sintaxe de signals (`.get()`, `.set()`) mudou para chamadas de função ou macros (`view!`). Devemos garantir compatibilidade com a versão instalada.
- **Performance:** Muitos dados no dashboard sem paginação podem impactar a renderização; usaremos limites nos seeds e queries.
