## Why

A interface atual do gerenciador de ativos carece de recursos básicos de usabilidade e visualização de dados. O cabeçalho não exibe informações do usuário logado, o menu lateral é estático e ocupa espaço desnecessário, e o dashboard principal precisa de uma apresentação mais profissional e dados realistas (seeds) para demonstração e testes eficazes.

## What Changes

- **Cabeçalho:** Inclusão de informações de usuário (nome e e-mail) no lado direito, botão de toggle para o menu lateral no lado esquerdo e link para visualização de perfil.
- **Menu Lateral:** Implementação de comportamento colapsável, inclusão de ícones para identificação visual rápida e botão de logout fixo na parte inferior.
- **Dashboard:** Novo layout profissional focado em gestão de ativos, utilizando componentes visuais modernos e alimentado por um novo sistema de seeds para entidades existentes.

## Capabilities

### New Capabilities

- `dashboard-seeding`: Mecanismo de geração de dados realistas para popular o dashboard de ativos.

### Modified Capabilities

- `web-admin-shell`: Atualização do cabeçalho com toggle de menu e informações de perfil/usuário.
- `auth-web`: Inclusão de logout no menu lateral e visualização de dados do usuário logado no header.
- `assets-web`: Novo dashboard profissional para gestão de ativos.

## Impact

- **Frontend (web-leptos):** Alterações significativas nos componentes de layout (Header, Sidebar) e na página de Dashboard.
- **Backend (api):** Pequenos ajustes se novos endpoints de perfil forem necessários (ou apenas consumo dos dados JWT existentes).
- **Infraestrutura:** Scripts de migration/seed para popular o banco com dados de teste.
