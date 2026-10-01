-- The NSS item is short runway coverage, not a complete show recording.
UPDATE assets
SET video_type = 'REEL_SHORT',
    completeness = 'PARTIAL',
    coverage_type = 'EDITORIAL',
    coverage_scope = 'RUNWAY_COVERAGE_REEL',
    officiality = 'PROFESSIONAL_VERIFIED',
    playback_mode = 'EXTERNAL_LINK',
    display_mode = 'LINK_ONLY',
    availability_status = 'REMOVED',
    updated_at = '2026-09-30T00:00:00.000Z'
WHERE id = 'asset-julie-kegels-ss27-reel';
