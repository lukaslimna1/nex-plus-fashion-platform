-- Verified Getty AAT batch. Labels and URIs were returned by the Getty SPARQL endpoint;
-- translations and definitions remain empty until reviewed.
INSERT OR IGNORE INTO terms (id, value, language, definition, source_ids, slug, canonical_name, pt_br_name, international_name, aliases_json, context, category, external_uri, related_term_ids, examples_json, retrieved_at)
VALUES
  ('term-getty-library-jacquard', 'jacquard', 'en', NULL, 'source-getty-aat', 'jacquard', 'jacquard', NULL, 'jacquard', '[]', NULL, 'technique', 'http://vocab.getty.edu/aat/300256616', '', '[]', '2026-09-30T00:00:00.000Z'),
  ('term-getty-library-tweed', 'tweed', 'en', NULL, 'source-getty-aat', 'tweed', 'tweed', NULL, 'tweed', '[]', NULL, 'material', 'http://vocab.getty.edu/aat/300132867', '', '[]', '2026-09-30T00:00:00.000Z'),
  ('term-getty-library-organza', 'organza', 'en', NULL, 'source-getty-aat', 'organza', 'organza', NULL, 'organza', '[]', NULL, 'material', 'http://vocab.getty.edu/aat/300310123', '', '[]', '2026-09-30T00:00:00.000Z'),
  ('term-getty-library-chiffon', 'chiffon', 'en', NULL, 'source-getty-aat', 'chiffon', 'chiffon', NULL, 'chiffon', '[]', NULL, 'material', 'http://vocab.getty.edu/aat/300249449', '', '[]', '2026-09-30T00:00:00.000Z'),
  ('term-getty-library-denim', 'denim', 'en', NULL, 'source-getty-aat', 'denim', 'denim', NULL, 'denim', '[]', NULL, 'material', 'http://vocab.getty.edu/aat/300014068', '', '[]', '2026-09-30T00:00:00.000Z');

INSERT OR IGNORE INTO ai_enrichment_jobs (id, entity_type, entity_id, task, status, input_source_ids, context_json, review_status, scheduled_at, created_at, updated_at)
SELECT 'ai-term-' || id, 'TERM', id, 'translateToPtBr', 'PENDING', source_ids,
  json_object('entityType','TERM','entityId',id,'canonicalName',canonical_name,'originalLabel',value,'language',language,'sourceUrls',json_array(external_uri),'retrievedText','','currentState',json_object('category',category,'aliases',aliases_json),'allowedOutputSchema',json_object('type','object','properties',json_object('ptBrName',json_object('type','string'),'aliases',json_object('type','array')))),
  'PENDING_REVIEW', '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z'
FROM terms WHERE id LIKE 'term-getty-library-%';
