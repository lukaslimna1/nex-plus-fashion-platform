# Rota Global da Moda — importação editorial

O Notion é a autoridade editorial para as duas data sources:

- `collection://75611a9f-e782-471e-93d3-7b3dc3b12b1f` — cidades
- `collection://a73b6ff1-f05e-40d4-bab5-21bc140c85e3` — eventos

O snapshot versionado em `docs/research/route-global-notion.snapshot.json` preserva as propriedades atuais e o `page_last_edited_at` de cada página. O manifest `docs/research/route-global-covers.json` descreve somente a inventariação local de `References/Covers`, com `coverAssetKey` estável, correspondência exata/normalizada/alias ou fallback. O catálogo não contém caminhos absolutos da máquina.

## Pipeline

```text
Notion data sources
  -> snapshot com identidade da página
  -> parse/validate/normalize (packages/data/src/route-import.ts)
  -> sourceHash FNV-1a estável + NEW/UPDATED/UNCHANGED/REVIEW_REQUIRED
  -> SQL idempotente (upsert de regiões, países, fontes, cidades, eventos e relações)
  -> D1/SQLite
  -> /api/cities e /api/events
```

Atualizar o snapshot e gerar o SQL local:

```text
npm run import:route-global -- --input docs/research/route-global-notion.snapshot.json --covers docs/research/route-global-covers.json --report-out docs/research/route-global-gaps.json --sql-out tmp/route-global-import.sql
npx wrangler d1 execute nex-plus-fashion --local --file=tmp/route-global-import.sql
```

`migrations/0007_route_global_notion.sql` cria os campos de sincronização, pesquisa, fontes, relações e capas. `migrations/0008_route_global_notion_seed.sql` é o seed inicial derivado do snapshot atual; ele não remove Julie Kegels nem MAXHOSA AFRICA.

O relatório `docs/research/route-global-gaps.json` lista distribuição por região, estados de completude, fontes, relações, capas faltantes e avisos de relação não resolvida. Códigos ISO ausentes no Notion permanecem ausentes no contrato; o valor `NOTION_UNSET:<slug>` existe apenas como sentinela interna da tabela legada `countries` e não é exposto pela API.

## Contrato para Antigravity

Os shapes finais estão em `@nex-plus/types`: `HomeResponse`/`HomeRail`, `Collection`/`CollectionDetail`, `Maison`/`MaisonDetail`, `CityHub`, `Event`, `Edition`, `ScheduleEntry`, `Asset`, `Source`, `ProfessionalReview`, `Term` e `Tag`. A API mantém envelopes `{ data, meta }`; cidades e eventos são listas completas, sem truncamento por quantidade fixa. Os filtros não alteram a forma dos itens.

Capas matched usam URL runtime em `/assets/covers/...`; entidades sem correspondência usam `cover-system-em-breve` e `fallback: true`. As imagens copiadas para `assets/covers` são derivadas dos arquivos fornecidos em `References/Covers`; os originais não são renomeados, recomprimidos ou sobrescritos.

## Cloudflare e Gemini

O repositório não contém credenciais. Se o ambiente do operador ainda não estiver autenticado, executar em terminal interativo:

```text
npx wrangler login
npx wrangler d1 list
npm run verify:infra
```

Depois de identificar o D1 existente, o operador deve preencher o `database_id` real em `wrangler.jsonc` (inclusive `preview`/`production` quando aplicável) e executar:

```text
npm run db:migrate:remote
npx wrangler secret put GEMINI_API_KEY --env production
npm run verify:ai
```

Nenhum desses passos remotos foi executado nesta rodada sem a credencial/ID real.
