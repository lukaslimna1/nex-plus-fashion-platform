CREATE VIRTUAL TABLE IF NOT EXISTS catalog_fts USING fts5(
    entity_id UNINDEXED,
    display_name,
    entity_kind,
    search_text
);

CREATE TRIGGER IF NOT EXISTS catalog_entity_ai AFTER INSERT ON catalog_entity BEGIN
    INSERT INTO catalog_fts(rowid, entity_id, display_name, entity_kind, search_text)
    VALUES (new.rowid, new.id, new.display_name, new.entity_kind, new.search_text);
END;

CREATE TRIGGER IF NOT EXISTS catalog_entity_ad AFTER DELETE ON catalog_entity BEGIN
    DELETE FROM catalog_fts WHERE rowid = old.rowid;
END;

CREATE TRIGGER IF NOT EXISTS catalog_entity_au AFTER UPDATE ON catalog_entity BEGIN
    DELETE FROM catalog_fts WHERE rowid = old.rowid;
    INSERT INTO catalog_fts(rowid, entity_id, display_name, entity_kind, search_text)
    VALUES (new.rowid, new.id, new.display_name, new.entity_kind, new.search_text);
END;
