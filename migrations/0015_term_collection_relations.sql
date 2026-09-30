CREATE TABLE IF NOT EXISTS term_collections (
  term_id TEXT NOT NULL REFERENCES terms(id),
  collection_id TEXT NOT NULL REFERENCES collections(id),
  evidence TEXT NOT NULL,
  source_ids TEXT NOT NULL DEFAULT '',
  PRIMARY KEY (term_id, collection_id)
);

INSERT OR IGNORE INTO term_collections (term_id, collection_id, evidence, source_ids)
VALUES ('term-izihlwele-zivumile', 'collection-maxhosa-africa-ss27-2026', 'The collection record names Izihlwele Zivumile (The Ancestors Have Approved) as the SS27 collection context.', 'source-fhcm,source-vogue-runway,source-maxhosa-youtube');
