ALTER TABLE assets ADD COLUMN coverage_type_label TEXT;
ALTER TABLE assets ADD COLUMN canonical_url TEXT;
ALTER TABLE assets ADD COLUMN channel_name TEXT;
ALTER TABLE assets ADD COLUMN duration_seconds INTEGER;
ALTER TABLE assets ADD COLUMN published_at TEXT;
ALTER TABLE assets ADD COLUMN video_type TEXT;
ALTER TABLE assets ADD COLUMN completeness TEXT;
ALTER TABLE assets ADD COLUMN officiality TEXT;
ALTER TABLE assets ADD COLUMN look_number INTEGER;

ALTER TABLE collections ADD COLUMN venue_name TEXT;
ALTER TABLE collections ADD COLUMN presentation_format TEXT;
ALTER TABLE collections ADD COLUMN creative_director_at_collection TEXT;
ALTER TABLE collections ADD COLUMN context TEXT;
ALTER TABLE collections ADD COLUMN organizer TEXT;

ALTER TABLE professional_reviews ADD COLUMN author TEXT;
ALTER TABLE professional_reviews ADD COLUMN publication TEXT;
ALTER TABLE professional_reviews ADD COLUMN summary TEXT;

INSERT OR IGNORE INTO regions (id, name, slug, created_at, updated_at) VALUES ('region-africa', 'Africa', 'africa', '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z');
INSERT OR IGNORE INTO countries (id, region_id, name, iso_code, slug, created_at, updated_at) VALUES ('country-south-africa', 'region-africa', 'South Africa', 'ZA', 'south-africa', '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z');

INSERT OR IGNORE INTO sources (id, canonical_name, type, base_url, authority_tier, language, coverage_scope, country_id, access_mode, rights_notes, automation_notes, last_verified_at, active, created_at, updated_at)
VALUES
  ('source-maxhosa-official', 'MAXHOSA AFRICA official', 'MAISON_OFFICIAL', 'https://maxhosa.africa', 'A', 'en', 'maison,collection,creative-direction', 'country-south-africa', 'PUBLIC', 'Official house site; use as provenance and link, not as a blanket media license.', 'Re-check collection-specific publication before treating site media as SS27.', '2026-09-30T00:00:00.000Z', 1, '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z'),
  ('source-maxhosa-youtube', 'MAXHOSA AFRICA official YouTube', 'VIDEO_CHANNEL', 'https://www.youtube.com/@maxhosa', 'A', 'en', 'official-video,film,collection', 'country-south-africa', 'PUBLIC', 'Official channel; embed only unless a separate license is documented.', 'Keep provider asset IDs and distinguish official films from a complete runway recording.', '2026-09-30T00:00:00.000Z', 1, '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z'),
  ('source-reuters-connect', 'Reuters Connect', 'RUNWAY_COVERAGE', 'https://www.reutersconnect.com', 'B', 'en', 'runway,photography,credit', 'country-france', 'LOGIN_REQUIRED', 'Image is licensable; no public rehost or download permission was established.', 'Preserve page URL, Reuters credit and USN; display as LINK_ONLY until licensed.', '2026-09-30T00:00:00.000Z', 1, '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z');

INSERT OR IGNORE INTO maisons (id, name, slug, website_url, founded_year, artistic_direction, official_source_ids, created_at, updated_at)
VALUES ('maison-maxhosa-africa', 'MAXHOSA AFRICA', 'maxhosa-africa', 'https://maxhosa.africa', 2010, 'Laduma Ngxokolo', 'source-maxhosa-official,source-fhcm,source-maxhosa-youtube', '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z');

INSERT OR IGNORE INTO collections (id, maison_id, edition_id, name, slug, calendar_year, season_year, season_code, season_label, presented_on, canonical_status, source_ids, venue_name, presentation_format, creative_director_at_collection, context, created_at, updated_at)
VALUES ('collection-maxhosa-africa-ss27-2026', 'maison-maxhosa-africa', 'edition-pfw-womenswear-ss27-2026', 'MAXHOSA AFRICA — Womenswear Spring/Summer 2027', 'maxhosa-africa-womenswear-spring-summer-2027', 2026, 2027, 'SS27', 'Spring/Summer 2027', '2026-09-28', 'CANONICAL', 'source-fhcm,source-maxhosa-official,source-maxhosa-youtube,source-vogue-runway,source-reuters-connect', 'Salons and garden of the official residence of the South African ambassador to France', 'SHOW', 'Laduma Ngxokolo', 'Izihlwele Zivumile (The Ancestors Have Approved); the show included a performance by South African artist Sjava.', '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z');

INSERT OR IGNORE INTO schedule_entries (id, edition_id, event_id, segment_id, city_hub_id, title, format, start_time, end_time, timezone, verification_status, official_url, source_ids, created_at, updated_at)
VALUES ('schedule-maxhosa-africa-ss27-2026-09-28', 'edition-pfw-womenswear-ss27-2026', 'event-paris-fashion-week', 'segment-womenswear', 'city-paris', 'MAXHOSA AFRICA', 'SHOW', '2026-09-28T15:00:00.000Z', NULL, 'Europe/Paris', 'VERIFIED', 'https://www.fhcm.paris/en/paris-fashion-week/calendar', 'source-fhcm', '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z');

WITH image_data(sequence_number, photo_id) AS (
  VALUES
    (1, '6abaacb7700c78b6d347a5e3'),
    (2, '6abaacb77b040a8c763ed3ee'),
    (3, '6abaacb7b9079b5920b8bbb0'),
    (4, '6abaacb9b1f31e6603ccc19d'),
    (5, '6abaacb9b321c70cf692b4e9'),
    (6, '6abaacb9b9079b5920b8bbba'),
    (7, '6abaacbb924dc4d8cb478b4b'),
    (8, '6abaacbb8e97dd916a6a71fd'),
    (9, '6abaacbbdec967757505fcd4'),
    (10, '6abaacbd2d5d41c0bb0bfe4c'),
    (11, '6abaacbd56e4845150c9de92'),
    (12, '6abaacbd2400048d8f91afdd'),
    (13, '6abaacbfe6e0624b87841df7'),
    (14, '6abaacc091a5281115a9cb00'),
    (15, '6abaacc04231c4acb44add19'),
    (16, '6abaacc1733202d9a18124b2'),
    (17, '6abaacc27b040a8c763ed3f5'),
    (18, '6abaacc271d267ee5949d3c6'),
    (19, '6abaacc41f3f615519079bd0'),
    (20, '6abaacc5811bbf79904cb8cb'),
    (21, '6abaacc4294b20220ff2735f'),
    (22, '6abaacc671d267ee5949d3cd'),
    (23, '6abaacc7f30c25b8f4cab47a'),
    (24, '6abaacc7f9c58579c688da7a'),
    (25, '6abaacc9dd9834bf7b388fcf'),
    (26, '6abaacc98397ac73585d0ef1'),
    (27, '6abaacc9e6e0624b87841e09'),
    (28, '6abaaccb811bbf79904cb8d2'),
    (29, '6abaaccbb8aed7d01f9403e7'),
    (30, '6abaaccbf065c6bd9446b9d0'),
    (31, '6abaaccd0adea3eb987d52d7'),
    (32, '6abaaccd920f649f55bd1efb'),
    (33, '6abaaccd0f01b94eb02b88dd'),
    (34, '6abaaccf8ed52f408d456403'),
    (35, '6abaaccf56e4845150c9de99'),
    (36, '6abaaccf59bd003bacff5166'),
    (37, '6abaacd143797329187762fe'),
    (38, '6abaacd14231c4acb44add26'),
    (39, '6abaacd1300c902695b302e3'),
    (40, '6abaacd28c99ee8b9772cdef'),
    (41, '6abaacd46fd1aec03d845bff'),
    (42, '6abaacd31747dd461091d4dd'),
    (43, '6abaacd443a203f579914ef3'),
    (44, '6abaacd5311cf888ff81d09f'),
    (45, '6abaacd6473fc9ad76b1e958'),
    (46, '6abaacd6498102ef54bd6457'),
    (47, '6abaacd791a5281115a9cb07'),
    (48, '6abaacd9c5b275f078cd875a'),
    (49, '6abaacd959bd003bacff5173'),
    (50, '6abaacd91f3f615519079bd7')
)
INSERT OR IGNORE INTO assets (id, collection_id, title, source_page_url, remote_url, provider, provider_asset_id, thumbnail_url, alternative_urls, creator, photographer, credit_line, source_ids, rights_status, download_policy, display_mode, canonical_status, asset_kind, coverage_type, coverage_scope, copyright_holder, attribution_required, embed_allowed, remote_render_allowed, rehost_allowed, verified_at, sequence_number, look_number, created_at, updated_at)
SELECT
  'asset-maxhosa-africa-ss27-look-' || printf('%02d', sequence_number),
  'collection-maxhosa-africa-ss27-2026',
  'MAXHOSA AFRICA SS27 — look ' || printf('%02d', sequence_number),
  'https://www.vogue.com/fashion-shows/spring-2027-ready-to-wear/maxhosa/slideshow/collection',
  'https://assets.vogue.com/photos/' || photo_id || '/master/w_1600,c_limit/' || printf('%05d', sequence_number) || '-maxhosa-spring-2027-ready-to-wear-credit-gorunway.jpg',
  'Vogue Runway',
  photo_id || '/' || printf('%05d', sequence_number) || '-maxhosa-spring-2027-ready-to-wear-credit-gorunway.jpg',
  'https://assets.vogue.com/photos/' || photo_id || '/master/w_360%2Cc_limit/' || printf('%05d', sequence_number) || '-maxhosa-spring-2027-ready-to-wear-credit-gorunway.jpg',
  'https://assets.vogue.com/photos/' || photo_id || '/master/w_960,c_limit/' || printf('%05d', sequence_number) || '-maxhosa-spring-2027-ready-to-wear-credit-gorunway.jpg',
  'Vogue Runway / Gorunway.com',
  'Filippo Fior',
  'Filippo Fior / Gorunway.com',
  'source-vogue-runway',
  'UNKNOWN', 'DOWNLOAD_BLOCKED', 'INLINE', 'CANONICAL', 'IMAGE', 'RUNWAY', 'LOOKBOOK_IMAGE',
  'Vogue Runway / Gorunway.com', 1, 0, 1, 0, '2026-09-30T00:00:00.000Z', sequence_number, sequence_number,
  '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z'
FROM image_data;

INSERT OR IGNORE INTO assets (id, collection_id, title, source_page_url, provider, provider_asset_id, alternative_urls, creator, photographer, credit_line, source_ids, rights_status, download_policy, display_mode, canonical_status, asset_kind, coverage_type, coverage_scope, copyright_holder, attribution_required, embed_allowed, remote_render_allowed, rehost_allowed, verified_at, sequence_number, canonical_url, channel_name, duration_seconds, published_at, video_type, completeness, officiality, created_at, updated_at)
VALUES ('asset-maxhosa-africa-ss27-official-film', 'collection-maxhosa-africa-ss27-2026', 'MAXHOSA AFRICA SS2027 — Izihlwele Zivumile', 'https://www.youtube.com/watch?v=yFus8VAaRME', 'YouTube', 'yFus8VAaRME', 'https://www.youtube.com/watch?v=Kl3OGBtMNl4', 'MAXHOSA AFRICA', NULL, 'Published on the MAXHOSA AFRICA official YouTube channel', 'source-maxhosa-youtube', 'UNKNOWN', 'DOWNLOAD_BLOCKED', 'EMBED', 'CANONICAL', 'VIDEO', 'EDITORIAL', 'EDITORIAL', 'MAXHOSA AFRICA', 1, 1, 0, 0, '2026-09-30T00:00:00.000Z', 200, 'https://www.youtube.com/watch?v=yFus8VAaRME', 'MAXHOSA AFRICA', 3176, '2026-09-29T17:42:21.000Z', 'OFFICIAL_FILM', 'UNKNOWN', 'OFFICIAL', '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z');
UPDATE assets SET embed_url = 'https://www.youtube.com/embed/yFus8VAaRME', thumbnail_url = 'https://i.ytimg.com/vi/yFus8VAaRME/hqdefault.jpg' WHERE id = 'asset-maxhosa-africa-ss27-official-film';

INSERT OR IGNORE INTO assets (id, collection_id, title, source_page_url, provider, provider_asset_id, creator, photographer, credit_line, source_ids, rights_status, download_policy, display_mode, canonical_status, asset_kind, coverage_type, coverage_scope, copyright_holder, license_name, attribution_required, embed_allowed, remote_render_allowed, rehost_allowed, verified_at, sequence_number, canonical_url, created_at, updated_at)
VALUES ('asset-maxhosa-africa-ss27-reuters-context', 'collection-maxhosa-africa-ss27-2026', 'MAXHOSA AFRICA SS27 — contextual runway photograph', 'https://www.reutersconnect.com/item/maxhosa-africa-womenswear-springsummer-2027-collection-during-paris-fashion-week/dGFnOnJldXRlcnMuY29tLDIwMjY6bmV3c21sX1JDMkdTTkFUNFdNOA', 'Reuters Connect', 'RC2GSNAT4WM8', 'REUTERS', 'Sarah Meyssonnier', 'REUTERS/Sarah Meyssonnier TPX IMAGES OF THE DAY', 'source-reuters-connect', 'RESTRICTED', 'DOWNLOAD_BLOCKED', 'LINK_ONLY', 'CANONICAL', 'IMAGE', 'RUNWAY', 'UNKNOWN', 'REUTERS', 'Reuters Connect transactional license', 1, 0, 0, 0, '2026-09-30T00:00:00.000Z', 100, 'https://www.reutersconnect.com/item/maxhosa-africa-womenswear-springsummer-2027-collection-during-paris-fashion-week/dGFnOnJldXRlcnMuY29tLDIwMjY6bmV3c21sX1JDMkdTTkFUNFdNOA', '2026-09-30T00:00:00.000Z', '2026-09-30T00:00:00.000Z');

INSERT OR IGNORE INTO asset_sources (asset_id, source_id, contribution, checked_at)
SELECT id, 'source-vogue-runway', 'ordered runway image, remote variants and Filippo Fior / Gorunway.com credit', '2026-09-30T00:00:00.000Z'
FROM assets
WHERE collection_id = 'collection-maxhosa-africa-ss27-2026' AND provider = 'Vogue Runway' AND asset_kind = 'IMAGE';
INSERT OR IGNORE INTO asset_sources (asset_id, source_id, contribution, checked_at) VALUES ('asset-maxhosa-africa-ss27-official-film', 'source-maxhosa-youtube', 'official channel upload; classified as film with unknown completeness', '2026-09-30T00:00:00.000Z');
INSERT OR IGNORE INTO asset_sources (asset_id, source_id, contribution, checked_at) VALUES ('asset-maxhosa-africa-ss27-reuters-context', 'source-reuters-connect', 'licensed runway photograph, Sarah Meyssonnier byline and Reuters USN RC2GSNAT4WM8', '2026-09-30T00:00:00.000Z');

INSERT OR IGNORE INTO professional_reviews (id, collection_id, source_id, title, url, published_at, language, canonical_status, author, publication, summary)
VALUES ('review-vogue-maxhosa-africa-ss27', 'collection-maxhosa-africa-ss27-2026', 'source-vogue-runway', 'Maxhosa Spring 2027 Ready-to-Wear Collection', 'https://www.vogue.com/fashion-shows/spring-2027-ready-to-wear/maxhosa', '2026-09-29T00:00:00.000Z', 'en', 'CANONICAL', 'Tina Isaac-Goizé', 'Vogue Runway', 'Professional review metadata; covers MAXHOSA AFRICA’s official Paris runway debut, the ambassador residence venue and Sjava performance.');

INSERT OR IGNORE INTO terms (id, value, language, definition, source_ids)
VALUES ('term-izihlwele-zivumile', 'Izihlwele Zivumile', 'en', 'The Ancestors Have Approved', 'source-fhcm,source-vogue-runway,source-maxhosa-youtube');
INSERT OR IGNORE INTO tags (id, value, slug, source_ids)
VALUES ('tag-maxhosa-africa-heritage', 'African heritage', 'african-heritage', 'source-fhcm,source-vogue-runway');
INSERT OR IGNORE INTO tags (id, value, slug, source_ids)
VALUES ('tag-maxhosa-knitwear', 'Knitwear', 'knitwear', 'source-vogue-runway');
INSERT OR IGNORE INTO collection_tags (collection_id, tag_id, evidence)
VALUES ('collection-maxhosa-africa-ss27-2026', 'tag-maxhosa-africa-heritage', 'FHCM interview and Vogue Runway review'), ('collection-maxhosa-africa-ss27-2026', 'tag-maxhosa-knitwear', 'Vogue Runway review');

CREATE INDEX IF NOT EXISTS idx_assets_collection_filters ON assets(collection_id, asset_kind, coverage_type, sequence_number);
CREATE INDEX IF NOT EXISTS idx_assets_provider_asset ON assets(provider, provider_asset_id);
