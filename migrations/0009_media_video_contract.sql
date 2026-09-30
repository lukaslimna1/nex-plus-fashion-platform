ALTER TABLE assets ADD COLUMN width INTEGER;
ALTER TABLE assets ADD COLUMN height INTEGER;
ALTER TABLE assets ADD COLUMN aspect_ratio TEXT;
ALTER TABLE assets ADD COLUMN orientation TEXT CHECK (orientation IN ('LANDSCAPE','PORTRAIT','SQUARE','UNKNOWN'));
ALTER TABLE assets ADD COLUMN playback_mode TEXT CHECK (playback_mode IN ('YOUTUBE_EMBED','INSTAGRAM_EMBED','VIMEO_EMBED','TIKTOK_EMBED','FACEBOOK_EMBED','WEBSITE_EMBED','HTML5_VIDEO','EXTERNAL_LINK'));
ALTER TABLE assets ADD COLUMN language TEXT;
ALTER TABLE assets ADD COLUMN availability_status TEXT CHECK (availability_status IN ('AVAILABLE','REGION_RESTRICTED','REMOVED','UNKNOWN'));
ALTER TABLE assets ADD COLUMN uploader_name TEXT;
ALTER TABLE assets ADD COLUMN uploader_url TEXT;
ALTER TABLE assets ADD COLUMN metadata_json TEXT;

CREATE TABLE IF NOT EXISTS media_research_jobs (
  id TEXT PRIMARY KEY,
  collection_id TEXT NOT NULL REFERENCES collections(id),
  source_id TEXT NOT NULL REFERENCES sources(id),
  media_type TEXT NOT NULL CHECK (media_type IN ('IMAGE','VIDEO')),
  status TEXT NOT NULL CHECK (status IN ('PENDING','RUNNING','SUCCEEDED','FAILED','BLOCKED')),
  last_attempt_at TEXT,
  next_eligible_attempt_at TEXT,
  result_count INTEGER NOT NULL DEFAULT 0,
  error TEXT,
  metadata_json TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  UNIQUE(collection_id, source_id, media_type)
);

CREATE INDEX IF NOT EXISTS idx_media_research_jobs_status ON media_research_jobs(status, next_eligible_attempt_at);
CREATE INDEX IF NOT EXISTS idx_assets_video_provider ON assets(asset_kind, provider, provider_asset_id);
