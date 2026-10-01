-- Promote FOUND matrix cells from registry entrypoints to the exact page
-- carried by the canonical asset relation. No new URL is inferred here.
UPDATE media_research_matrix AS matrix
SET source_page_url = (
  SELECT MIN(a.source_page_url)
  FROM assets a
  WHERE a.collection_id = matrix.collection_id
    AND a.asset_kind = matrix.media_type
    AND (
      (matrix.source_key = 'FHCM' AND instr(',' || a.source_ids || ',', ',source-fhcm,') > 0)
      OR (matrix.source_key = 'VOGUE' AND instr(',' || a.source_ids || ',', ',source-vogue-runway,') > 0)
      OR (matrix.source_key = 'OUI' AND instr(',' || a.source_ids || ',', ',source-oui-speak-fashion,') > 0)
      OR (matrix.source_key IN ('NOWFASHION','TAGWALK','FF_CHANNEL','FASHION_CHANNEL') AND instr(',' || a.source_ids || ',', ',' || matrix.source_id || ',') > 0)
    )
)
WHERE matrix.state = 'FOUND';
