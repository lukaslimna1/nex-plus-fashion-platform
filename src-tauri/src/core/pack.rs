use crate::core::db::{CatalogDb, PackRuntimeRow};
use crate::core::error::CoreError;
use crate::core::milano::{
    bundled_pack, NexPack, NexPackManifest, PackAppCompatibility, PackOrigin, PackScope,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

const BUNDLED_PACK_TEXT: &str = include_str!("../../../packs/milano-ss27-2026.10.03.nexpack");
pub const BUNDLED_PACK_ID: &str = "nex.fashion.milano.ss27";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackSummary {
    pub pack_id: String,
    pub version: String,
    pub family: String,
    pub title: String,
    pub scope: PackScope,
    pub schema_version: String,
    pub app_compatibility: PackAppCompatibility,
    pub content_hash: String,
    pub artifact_hash: Option<String>,
    pub size_bytes: u64,
    pub origin: PackOrigin,
    pub entity_counts: std::collections::BTreeMap<String, usize>,
    pub state: String,
    pub progress: u8,
    pub available: bool,
    pub update_available: bool,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackOperationResult {
    pub operation: String,
    pub pack: PackSummary,
}

pub struct PackRuntime {
    root: PathBuf,
}

impl PackRuntime {
    pub fn new(root: impl AsRef<Path>) -> Result<Self, CoreError> {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(root.join("staging"))?;
        fs::create_dir_all(root.join("installed"))?;
        Ok(Self { root })
    }

    pub fn ensure_available(&self, database: &mut CatalogDb) -> Result<(), CoreError> {
        let pack = bundled_pack()?;
        database.ensure_pack_available(&pack, BUNDLED_PACK_TEXT.len() as u64)
    }

    pub fn list(&self, database: &mut CatalogDb) -> Result<Vec<PackSummary>, CoreError> {
        let rows = database.pack_runtime_rows()?;
        let mut summaries = Vec::with_capacity(rows.len());
        for row in rows {
            if row.state == "active" && !self.installed_artifact_is_valid(&row)? {
                database.set_pack_runtime_state(
                    &row.pack_id,
                    &row.version,
                    "repair_required",
                    0,
                    None,
                    None,
                    None,
                    None,
                    Some("installed artifact is missing or corrupt"),
                )?;
                summaries.push(self.summary(&database.pack_runtime_row(&row.pack_id)?)?);
            } else {
                summaries.push(self.summary(&row)?);
            }
        }
        Ok(summaries)
    }

    pub fn install(
        &self,
        database: &mut CatalogDb,
        pack_id: &str,
    ) -> Result<PackOperationResult, CoreError> {
        self.stage(database, pack_id)?;
        self.verify_staged(database, pack_id)?;

        let pack = bundled_pack()?;
        database.set_pack_runtime_state(
            pack_id,
            &pack.manifest.version,
            "installing",
            75,
            None,
            None,
            None,
            None,
            None,
        )?;

        let row = database.pack_runtime_row(pack_id)?;
        let staged_path = row
            .staged_path
            .as_deref()
            .ok_or_else(|| CoreError::Pack("verified pack has no staging path".to_string()))?;
        let installed_path = self.installed_path(&pack);
        let temporary_path = installed_path.with_extension("nexpack.part");
        if let Err(error) = fs::copy(staged_path, &temporary_path) {
            return self.fail(database, pack_id, &pack, "error", &error.to_string());
        }

        if let Err(error) = database.import_milano_pack(&pack) {
            let _ = fs::remove_file(&temporary_path);
            return self.fail(database, pack_id, &pack, "error", &error.to_string());
        }

        if let Err(error) = fs::rename(&temporary_path, &installed_path) {
            let _ = fs::remove_file(&temporary_path);
            return self.fail(
                database,
                pack_id,
                &pack,
                "repair_required",
                &error.to_string(),
            );
        }

        let artifact_hash = sha256_bytes(BUNDLED_PACK_TEXT.as_bytes());
        database.set_pack_runtime_state(
            pack_id,
            &pack.manifest.version,
            "active",
            100,
            Some(staged_path),
            Some(path_string(&installed_path).as_str()),
            Some(&artifact_hash),
            Some(BUNDLED_PACK_TEXT.len() as u64),
            None,
        )?;
        let summary = self.summary(&database.pack_runtime_row(pack_id)?)?;
        Ok(PackOperationResult {
            operation: "install".to_string(),
            pack: summary,
        })
    }

    pub fn remove(
        &self,
        database: &mut CatalogDb,
        pack_id: &str,
    ) -> Result<PackOperationResult, CoreError> {
        let row = database.pack_runtime_row(pack_id)?;
        database.remove_pack(pack_id)?;
        database.set_pack_runtime_state(
            pack_id,
            &row.version,
            "removed",
            0,
            None,
            None,
            row.artifact_hash.as_deref(),
            Some(row.size_bytes),
            None,
        )?;
        let summary = self.summary(&database.pack_runtime_row(pack_id)?)?;
        Ok(PackOperationResult {
            operation: "remove".to_string(),
            pack: summary,
        })
    }

    pub fn repair(
        &self,
        database: &mut CatalogDb,
        pack_id: &str,
    ) -> Result<PackOperationResult, CoreError> {
        let row = database.pack_runtime_row(pack_id)?;
        if row.state == "active" && self.installed_artifact_is_valid(&row)? {
            return Ok(PackOperationResult {
                operation: "repair".to_string(),
                pack: self.summary(&row)?,
            });
        }
        let result = self.install(database, pack_id)?;
        Ok(PackOperationResult {
            operation: "repair".to_string(),
            pack: result.pack,
        })
    }

    fn stage(&self, database: &mut CatalogDb, pack_id: &str) -> Result<(), CoreError> {
        let pack = bundled_pack()?;
        if pack_id != pack.manifest.pack_id || pack_id != BUNDLED_PACK_ID {
            return Err(CoreError::PackNotFound(pack_id.to_string()));
        }
        let path = self.staging_path(&pack);
        database.set_pack_runtime_state(
            pack_id,
            &pack.manifest.version,
            "staged",
            15,
            None,
            None,
            None,
            Some(BUNDLED_PACK_TEXT.len() as u64),
            None,
        )?;
        if let Err(error) = fs::write(&path, BUNDLED_PACK_TEXT.as_bytes()) {
            return self.fail(database, pack_id, &pack, "error", &error.to_string());
        }
        let artifact_hash = sha256_bytes(BUNDLED_PACK_TEXT.as_bytes());
        database.set_pack_runtime_state(
            pack_id,
            &pack.manifest.version,
            "staged",
            30,
            Some(path_string(&path).as_str()),
            None,
            Some(&artifact_hash),
            Some(BUNDLED_PACK_TEXT.len() as u64),
            None,
        )?;
        Ok(())
    }

    fn verify_staged(&self, database: &mut CatalogDb, pack_id: &str) -> Result<(), CoreError> {
        let row = database.pack_runtime_row(pack_id)?;
        let path = row
            .staged_path
            .as_deref()
            .ok_or_else(|| CoreError::Pack("pack is not staged".to_string()))?;
        database.set_pack_runtime_state(
            pack_id,
            &row.version,
            "verifying",
            50,
            None,
            None,
            None,
            None,
            None,
        )?;
        let result = (|| {
            let bytes = fs::read(path)?;
            let artifact_hash = sha256_bytes(&bytes);
            if row.artifact_hash.as_deref() != Some(artifact_hash.as_str()) {
                return Err(CoreError::Pack("staged artifact hash mismatch".to_string()));
            }
            let pack: NexPack = serde_json::from_slice(&bytes)?;
            pack.verify()?;
            if pack.manifest.pack_id != pack_id || pack.manifest.version != row.version {
                return Err(CoreError::Pack("staged pack identity mismatch".to_string()));
            }
            Ok(())
        })();
        if let Err(error) = result {
            return self.fail(
                database,
                pack_id,
                &bundled_pack()?,
                "repair_required",
                &error.to_string(),
            );
        }
        database.set_pack_runtime_state(
            pack_id,
            &row.version,
            "verified",
            65,
            None,
            None,
            None,
            None,
            None,
        )?;
        Ok(())
    }

    fn fail<T>(
        &self,
        database: &mut CatalogDb,
        pack_id: &str,
        pack: &NexPack,
        state: &str,
        message: &str,
    ) -> Result<T, CoreError> {
        database.set_pack_runtime_state(
            pack_id,
            &pack.manifest.version,
            state,
            0,
            None,
            None,
            None,
            None,
            Some(message),
        )?;
        Err(CoreError::Pack(message.to_string()))
    }

    fn summary(&self, row: &PackRuntimeRow) -> Result<PackSummary, CoreError> {
        let manifest: NexPackManifest = serde_json::from_str(&row.manifest_json)?;
        Ok(PackSummary {
            pack_id: manifest.pack_id.clone(),
            version: manifest.version.clone(),
            family: manifest.family.clone(),
            title: manifest.title.clone(),
            scope: manifest.scope.clone(),
            schema_version: manifest.schema_version.clone(),
            app_compatibility: manifest.app_compatibility.clone(),
            content_hash: manifest.content_hash.clone(),
            artifact_hash: row.artifact_hash.clone(),
            size_bytes: manifest.size_bytes,
            origin: manifest.origin.clone(),
            entity_counts: manifest.entity_counts.clone(),
            state: row.state.clone(),
            progress: row.progress,
            available: true,
            update_available: row.version != manifest.version,
            last_error: row.last_error.clone(),
        })
    }

    fn installed_artifact_is_valid(&self, row: &PackRuntimeRow) -> Result<bool, CoreError> {
        let Some(path) = row.installed_path.as_deref() else {
            return Ok(false);
        };
        let bytes = match fs::read(path) {
            Ok(bytes) => bytes,
            Err(_) => return Ok(false),
        };
        if let Some(expected) = row.artifact_hash.as_deref() {
            if sha256_bytes(&bytes) != expected {
                return Ok(false);
            }
        }
        let pack: NexPack = match serde_json::from_slice(&bytes) {
            Ok(pack) => pack,
            Err(_) => return Ok(false),
        };
        Ok(pack.verify().is_ok()
            && pack.manifest.pack_id == row.pack_id
            && pack.manifest.version == row.version)
    }

    fn staging_path(&self, pack: &NexPack) -> PathBuf {
        self.root.join("staging").join(format!(
            "{}-{}.nexpack",
            pack.manifest.pack_id, pack.manifest.version
        ))
    }

    fn installed_path(&self, pack: &NexPack) -> PathBuf {
        self.root.join("installed").join(format!(
            "{}-{}.nexpack",
            pack.manifest.pack_id, pack.manifest.version
        ))
    }
}

fn sha256_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::{PackRuntime, BUNDLED_PACK_ID};
    use crate::core::db::CatalogDb;
    use std::fs;

    fn temp_root() -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "nex-fashion-pack-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock should be valid")
                .as_nanos()
        ))
    }

    #[test]
    fn milano_pack_completes_install_remove_reinstall_and_repair_cycle() {
        let root = temp_root();
        let runtime = PackRuntime::new(&root).expect("runtime should initialize");
        let mut database = CatalogDb::in_memory().expect("database should initialize");

        runtime
            .ensure_available(&mut database)
            .expect("pack should be available");
        assert_eq!(runtime.list(&mut database).unwrap()[0].state, "available");

        runtime
            .install(&mut database, BUNDLED_PACK_ID)
            .expect("pack should install");
        assert_eq!(runtime.list(&mut database).unwrap()[0].state, "active");
        assert!(!database.search("Prada", 10).unwrap().is_empty());
        assert_eq!(
            database
                .pack_entity_count(BUNDLED_PACK_ID, "schedule_entry")
                .unwrap(),
            214
        );

        database
            .upsert_personal_note("note:pack-cycle", "maison:prada", "revisit this maison")
            .expect("note should be created");
        runtime
            .remove(&mut database, BUNDLED_PACK_ID)
            .expect("pack should be removed");
        assert_eq!(runtime.list(&mut database).unwrap()[0].state, "removed");
        assert!(database.search("Prada", 10).unwrap().is_empty());
        assert_eq!(
            database.personal_note("note:pack-cycle").unwrap().entity_id,
            "maison:prada"
        );

        runtime
            .install(&mut database, BUNDLED_PACK_ID)
            .expect("pack should reinstall");
        assert_eq!(runtime.list(&mut database).unwrap()[0].state, "active");
        assert!(!database.search("Prada", 10).unwrap().is_empty());
        assert_eq!(
            database.personal_note("note:pack-cycle").unwrap().entity_id,
            "maison:prada"
        );

        runtime
            .install(&mut database, BUNDLED_PACK_ID)
            .expect("repeated install should be idempotent");
        assert_eq!(
            database
                .pack_entity_count(BUNDLED_PACK_ID, "schedule_entry")
                .unwrap(),
            214
        );

        runtime
            .stage(&mut database, BUNDLED_PACK_ID)
            .expect("pack should stage");
        let staged_path = database
            .pack_runtime_row(BUNDLED_PACK_ID)
            .unwrap()
            .staged_path
            .expect("staged path should be recorded");
        fs::write(&staged_path, b"corrupt staged pack")
            .expect("staged artifact should be corruptible");
        assert!(runtime
            .verify_staged(&mut database, BUNDLED_PACK_ID)
            .is_err());
        assert_eq!(
            runtime.list(&mut database).unwrap()[0].state,
            "repair_required"
        );
        assert!(!database.search("Prada", 10).unwrap().is_empty());
        runtime
            .repair(&mut database, BUNDLED_PACK_ID)
            .expect("repair should preserve materialization after failed verify");
        assert_eq!(runtime.list(&mut database).unwrap()[0].state, "active");

        let installed_path = database
            .pack_runtime_row(BUNDLED_PACK_ID)
            .unwrap()
            .installed_path
            .expect("installed path should be recorded");
        fs::write(&installed_path, b"corrupt pack").expect("artifact should be corruptible");
        assert_eq!(
            runtime.list(&mut database).unwrap()[0].state,
            "repair_required"
        );
        runtime
            .repair(&mut database, BUNDLED_PACK_ID)
            .expect("repair should recover the pack");
        assert_eq!(runtime.list(&mut database).unwrap()[0].state, "active");
        assert!(!database.search("Prada", 10).unwrap().is_empty());
        assert_eq!(
            database.personal_note("note:pack-cycle").unwrap().entity_id,
            "maison:prada"
        );

        drop(database);
        fs::remove_dir_all(&root).expect("pack test directory should be cleaned");
    }
}
