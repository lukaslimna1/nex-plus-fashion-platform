# Canonical data model

The D1 schema in `migrations/0001_core.sql` covers regions, countries, administrative areas, city hubs, events, locations, segments, event segments, editions, schedules, maisons, creative-direction history, collections, sources, assets, asset-source contributions, reviews, terms, tags, favorites, community data, sync revisions/tombstones, source checks and AI runs.

Calendar identity is intentionally split into `calendar_year`, `season_year`, `season_code` and `season_label`. For the first verified slice, a calendar event in 2026 belongs to the SS27 season year 2027. Paris Fashion Week and Haute Couture Week have different event identities; Womenswear is an EventSegment, not a replacement for the event.
