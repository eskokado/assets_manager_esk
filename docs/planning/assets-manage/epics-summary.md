# Resumo de Épicos — Assets Manage

**Perfil**: `docs/planning/assets-manage/delivery-profile.md`
**Stack**: Rust (Axum) · Leptos SSR · Nenhum mobile
**Data**: 2026-06-06

| ID | Épico | BC | Tamanho | Stories | Tasks (≈) | Dep. | Release |
|----|-------|-----|---------|---------|-----------|------|---------|
| EP-000 | Bootstrap e Infraestrutura | [TECH] | M | 1 | 12 | — | 0 |
| EP-001 | Auth — Identidade e Acesso | Auth | M | 2 | 28 | EP-000 | 1 |
| EP-002 | Assets — Catálogo | Assets | M | 2 | 30 | EP-001 | 1 |
| EP-003 | Trading — Negociação | Trading | G | 2 | 32 | EP-002 | 1 |
| EP-004 | Portfolio — Carteira | Portfolio | M | 2 | 26 | EP-003 | 1 |

**Totais**: 5 épicos (4 BCs + 1 técnico), 9 stories, ~128 tasks, ~180–220 horas estimadas

## Ordem de implementação

```
EP-000 → EP-001 → EP-002 → EP-003 → EP-004
                              └─ Portfolio materializado dentro de EP-003 (TX)
                                 EP-004 = queries + UI de leitura
```

## OpenSpec sugerido

| Mudança | Escopo |
|---------|--------|
| `bootstrap-assets-manage` | EP-000 |
| `ep-001-auth` | EP-001 |
| `ep-002-assets` | EP-002 |
| `ep-003-trading` | EP-003 |
| `ep-004-portfolio` | EP-004 |
