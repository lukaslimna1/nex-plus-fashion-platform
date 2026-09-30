# Workstreams — Codex x Antigravity

## Objetivo

Permitir trabalho paralelo sem colisão de arquivos e sem decisões divergentes.

## Codex — ownership

Pode editar:

- `apps/worker/**`
- `packages/core/**`
- `packages/data/**`
- `packages/types/**`
- `packages/media/**` quando for contrato/resolver/infra
- `scripts/**` para ingestão, scraping, migração e validação
- migrations, testes de domínio/dados e documentação técnica

Responsabilidades:

- catálogo canônico;
- modelo relacional;
- D1/SQLite;
- Source Registry;
- scraping/ingestão;
- schedule real-time;
- Gemini/extraction pipeline;
- API;
- sync;
- media resolver;
- proveniência;
- testes e integridade.

Não editar UI visual em `apps/web` ou `packages/ui`.

## Antigravity — ownership

Pode editar:

- `apps/web/**`
- `packages/ui/**`
- `packages/media/**` somente componentes visuais aprovados
- documentação de design/UX

Responsabilidades:

- design system;
- navbar;
- Home;
- Rota Global;
- Cidade/Event/Edition/Maison/Collection;
- Tendências;
- Biblioteca;
- Busca/Favoritos/Conta;
- player visual;
- Look Viewer;
- responsive Web/PWA;
- estados loading/empty/error;
- acessibilidade e animação.

Não editar schema, migrations, scraping, adapters ou D1.

## Arquivos compartilhados

- `packages/types/**` é ownership do Codex.
- Antigravity consome contratos, mas não altera sem combinar.
- `assets/**` é curado pelo projeto; nenhum agente deve substituir capas/logos sem autorização.
- arquivos root de workspace/package manager devem ser alterados por um agente de cada vez.

## Git

Branches recomendadas:

- Codex: `feat/data-platform-foundation`
- Antigravity: `feat/web-design-system-foundation`

Nenhum dos dois faz merge em `main`.
