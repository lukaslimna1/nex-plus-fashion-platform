# NEX+ Fashion

NEX+ Fashion é uma plataforma editorial audiovisual de moda, pensada como um acervo global, relacional e multiplataforma.

## Direção atual

- Web como frontend canônico.
- PWA derivada da mesma aplicação Web.
- Desktop reutiliza a mesma UI e adiciona capacidades locais, como Caderno/Minha Leitura, biblioteca local e reindexação.
- Catálogo online canônico, com mídia remota agregada por URL/embed e proveniência explícita.
- Cloudflare como infraestrutura inicial zero-cost-first.
- Notion é a base canônica de decisões editoriais e de produto.

## Estrutura

- `apps/web` — frontend Web/PWA.
- `apps/worker` — API/Worker e integração online.
- `packages/types` — contratos e tipos compartilhados.
- `packages/core` — domínio e regras puras.
- `packages/data` — repositories/adapters/sync.
- `packages/ui` — design system e componentes React compartilhados.
- `packages/media` — contratos/resolução de mídia.
- `assets` — assets próprios do NEX+ Fashion.
- `docs` — arquitetura, produto e contratos de trabalho.

Leia `AGENTS.md` e `docs/WORKSTREAMS.md` antes de alterar o projeto.
