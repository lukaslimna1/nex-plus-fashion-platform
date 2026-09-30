# Canonical data model

The D1 schema in `migrations/0001_core.sql` covers regions, countries, administrative areas, city hubs, events, locations, segments, event segments, editions, schedules, maisons, creative-direction history, collections, sources, assets, asset-source contributions, reviews, terms, tags, favorites, community data, sync revisions/tombstones, source checks and AI runs. `migrations/0009_media_video_contract.sql` adds explicit video geometry/playback fields and the resumable `media_research_jobs` queue.

Calendar identity is intentionally split into `calendar_year`, `season_year`, `season_code` and `season_label`. For the first verified slice, a calendar event in 2026 belongs to the SS27 season year 2027. Paris Fashion Week and Haute Couture Week have different event identities; Womenswear is an EventSegment, not a replacement for the event.

Media is modeled as `Asset` for compatibility with the existing catalog and as the stricter `VideoAsset` contract for video consumers. A video always carries a technical `provider`, optional `providerAssetId`, `playbackMode`, `orientation`, `aspectRatio`, availability and source provenance. A `FULL_SHOW` classification is only emitted when completeness is proven by the source; editorial coverage, reels and films remain non-full classifications.
