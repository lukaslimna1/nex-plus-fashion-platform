# Fase 1 baseline

This is the clean architectural baseline after the legacy implementation reset.

## Current boundary

- Tauri 2 is the desktop shell.
- React/TypeScript is the replaceable visual surface.
- Rust owns the local Core and the IPC command boundary.
- SQLite is the local storage boundary; FTS5 is prepared by migration `0002_fts5`.
- `packages/contracts` is the first typed contract package shared by the shell and Core-facing UI.

The baseline has no Milano facts, no editorial seed, no pack payload, and no cloud dependency. That is deliberate: the Fase 1 gate requires startup and compilation evidence before the Milano vertical is added.

## Domain boundary

The generic entity-kind boundary already names the canonical concepts needed by the first vertical: CityHub, Event, Edition, ScheduleEntry, Venue, Maison, Person, Collection, Source, MediaAsset, Review, and PersonalNote. It does not assert factual records or invent relationships. Those records will arrive through reviewed sources and Pack contracts in the next step.

## Reset evidence

The old application, database, migrations, assets, build output, caches, and legacy documentation were removed in the reset commit. The local `.env` files are ignored and remain outside Git. The reset was published to the existing repository branches without changing the repository identity or remote.
