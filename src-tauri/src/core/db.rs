use crate::core::error::CoreError;
use rusqlite::{Connection, OptionalExtension};

const MIGRATIONS: [(&str, &str); 2] = [
    ("0001_core", include_str!("../../migrations/0001_core.sql")),
    ("0002_fts5", include_str!("../../migrations/0002_fts5.sql")),
];

pub struct CatalogDb {
    connection: Connection,
}

impl CatalogDb {
    pub fn in_memory() -> Result<Self, CoreError> {
        let connection = Connection::open_in_memory()?;
        connection.pragma_update(None, "foreign_keys", "ON")?;
        let database = Self { connection };
        database.apply_migrations()?;
        Ok(database)
    }

    fn apply_migrations(&self) -> Result<(), CoreError> {
        self.connection.execute_batch("BEGIN IMMEDIATE")?;
        for (name, sql) in MIGRATIONS {
            self.connection.execute_batch(sql)?;
            self.connection.execute(
                "INSERT OR IGNORE INTO schema_migration (name) VALUES (?1)",
                [name],
            )?;
        }
        self.connection.execute_batch("COMMIT")?;
        Ok(())
    }

    pub fn migration_names(&self) -> Result<Vec<String>, CoreError> {
        let mut statement = self
            .connection
            .prepare("SELECT name FROM schema_migration ORDER BY name")?;
        let rows = statement.query_map([], |row| row.get(0))?;
        Ok(rows.collect::<Result<Vec<String>, _>>()?)
    }

    pub fn has_fts5(&self) -> Result<bool, CoreError> {
        let exists = self
            .connection
            .query_row(
                "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'catalog_fts'",
                [],
                |row| row.get::<_, i32>(0),
            )
            .optional()?;
        Ok(exists.is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::CatalogDb;

    #[test]
    fn baseline_migrations_create_sqlite_and_fts5() {
        let database = CatalogDb::in_memory().expect("baseline database should initialize");
        assert_eq!(database.migration_names().unwrap(), vec!["0001_core", "0002_fts5"]);
        assert!(database.has_fts5().unwrap());
    }
}
