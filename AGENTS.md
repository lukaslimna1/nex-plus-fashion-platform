# NEX+ Fashion — regras para agentes

## Fonte de verdade

1. Notion canônico do NEX+ Fashion.
2. Contratos e documentação versionados neste repositório.
3. Código e testes.
4. Implementações antigas servem apenas como referência, nunca como verdade automática.

## Regra de colaboração

Codex e Antigravity trabalham em branches/worktrees separados e NÃO editam os mesmos domínios ao mesmo tempo.

- Codex: arquitetura, dados, scraping, API, D1/SQLite, sync, contratos, testes e infraestrutura.
- Antigravity: Web/PWA, design system, UX/UI, responsividade, player/viewer e integração visual com contratos estáveis.

Leia `docs/WORKSTREAMS.md` para ownership por pasta.

## Restrições

- Não fazer merge em main sem aprovação explícita.
- Não inventar dados editoriais.
- Não adicionar mocks factuais como se fossem conteúdo real.
- Não armazenar segredos no repositório.
- Mídia de terceiros deve ser tratada por URL/embed/proveniência, não como asset próprio por padrão.
- Assets próprios do NEX+ ficam em `assets/`.
