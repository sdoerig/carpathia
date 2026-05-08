use crate::db::db_schema_structs::AbstractDbRepr;
use crate::return_values::carpathia_errors::{CarpathiaError, ErrorNumber};
use clap::ValueEnum;
use log::{error, info};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum CacheModus {
    BypassCache,
    UseCache,
}

pub(crate) struct CacheSectionDiff {
    pub to_generate: Vec<String>,
    pub to_remove: Vec<String>,
}
pub(crate) struct CacheFileDiff {
    pub tables: CacheSectionDiff,
    pub views: CacheSectionDiff,
}
impl CacheFileDiff {
    pub(crate) fn new() -> Self {
        CacheFileDiff {
            tables: CacheSectionDiff {
                to_generate: Vec::new(),
                to_remove: Vec::new(),
            },
            views: CacheSectionDiff {
                to_generate: Vec::new(),
                to_remove: Vec::new(),
            },
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct CacheFile {
    pub tables: BTreeMap<String, String>,
    pub views: BTreeMap<String, String>,
}

impl CacheFile {
    pub(crate) fn new() -> Self {
        CacheFile {
            tables: BTreeMap::new(),
            views: BTreeMap::new(),
        }
    }
    pub(crate) fn from_file(path: &PathBuf) -> Result<Self, CarpathiaError> {
        let file_content = std::fs::read_to_string(path).map_err(|e| CarpathiaError {
            message: format!("Failed to read cache file at {path:?}: {e}"),
            error_type: ErrorNumber::CacheFileReadError,
        })?;
        let cache_file = match serde_json::from_str(&file_content) {
            Ok(cache) => cache,
            Err(e) => {
                error!("Failed to parse cache file at {path:?}: {e}");
                return Err(CarpathiaError {
                    message: format!("Failed to parse cache file at {path:?}: {e}"),
                    error_type: ErrorNumber::CacheFileReadError,
                });
            }
        };
        Ok(cache_file)
    }

    pub(crate) fn from_abstract_db_repr(db_repr: &AbstractDbRepr) -> Self {
        let mut cache_file = CacheFile::new();
        for (table_name, table_repr) in &db_repr.tables {
            let table_hash = sha256_hash(table_repr);
            cache_file.tables.insert(table_name.clone(), table_hash);
        }
        for (view_name, view_repr) in &db_repr.views {
            let view_hash = sha256_hash(view_repr);
            cache_file.views.insert(view_name.clone(), view_hash);
        }
        cache_file
    }

    pub(crate) fn save_to_file(&self, path: &PathBuf) -> Result<(), CarpathiaError> {
        match fs::create_dir_all(path.parent().unwrap()) {
            Ok(()) => {
                //let cache_file_path = format!("{}/{}", &self.path, CACHE_FILE_NAME);
                let cache_content_json = serde_json::to_string_pretty(&self).unwrap();
                match fs::write(path, cache_content_json) {
                    Ok(()) => {
                        info!("Cache file updated successfully at {}", &path.display());
                        Ok(())
                    }
                    Err(e) => {
                        error!("Failed to write cache file: {e}");
                        Err(CarpathiaError {
                            message: "Failed to write cache file".to_string(),
                            error_type: ErrorNumber::CacheFileError,
                        })
                    }
                }
            }
            Err(e) => {
                error!("Failed to create cache directory: {e}");
                Err(CarpathiaError {
                    message: "Failed to create cache directory".to_string(),
                    error_type: ErrorNumber::CacheFileError,
                })
            }
        }
    }
}

pub(crate) fn compare_cache_files(
    old_cache: &CacheFile,
    new_cache: &CacheFile,
    cache_usage: CacheModus,
) -> CacheFileDiff {
    let mut cache_diff = CacheFileDiff::new();
    diff_btrees(
        &old_cache.tables,
        &new_cache.tables,
        cache_usage,
        &mut cache_diff.tables,
    );
    diff_btrees(
        &old_cache.views,
        &new_cache.views,
        cache_usage,
        &mut cache_diff.views,
    );
    cache_diff
}

fn diff_btrees(
    old_cache: &BTreeMap<String, String>,
    new_cache: &BTreeMap<String, String>,
    cache_usage: CacheModus,
    cache_diff: &mut CacheSectionDiff,
) {
    // Zu entfernende Elemente: In Old, aber nicht in New
    cache_diff.to_remove.extend(
        old_cache
            .keys()
            .filter(|k| !new_cache.contains_key(*k))
            .cloned(),
    );

    // Zu generierende Elemente: Neu, geändert oder 'force'
    cache_diff.to_generate.extend(
        new_cache
            .iter()
            .filter(|(k, new_hash)| {
                cache_usage == CacheModus::BypassCache || old_cache.get(*k) != Some(*new_hash)
            })
            .map(|(k, _)| k.clone()),
    );
}

fn sha256_hash<T: Serialize>(item: &T) -> String {
    let json_string = serde_json::to_string(item).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(json_string.as_bytes());
    let hash_result = hasher.finalize();
    format!("{hash_result:x}")
}
