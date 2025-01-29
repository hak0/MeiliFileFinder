use crate::config::{MeiliSearchConfig, ProjectConfig};
use crate::file_index::{FileSystemEntry, IndexEntryType};
use chrono::{DateTime, Utc};
use ignore::WalkBuilder;
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[cfg(test)]
#[path = "tests/indexer_tests.rs"]
mod file_index_tests;

#[derive(Debug, Clone)]
pub struct Indexer {
    pub project_config: ProjectConfig,
    pub meili_client: Option<meilisearch_sdk::client::Client>,
}

impl Indexer {
    // Create a new Indexer instance
    pub fn new(project_config: &ProjectConfig, meilisearch_config: &MeiliSearchConfig) -> Self {
        let meilisearch_url = &meilisearch_config.meilisearch_url;
        let meilisearch_api_key = &meilisearch_config.meilisearch_api_key;
        if meilisearch_url.is_empty() || meilisearch_api_key.is_empty() {
            eprintln!("Meilisearch URL or API key is empty. Exiting.");
            std::process::exit(1);
        }
        let meili_client =
            meilisearch_sdk::client::Client::new(meilisearch_url, Some(meilisearch_api_key)).ok();

        Indexer {
            project_config: project_config.clone(),
            meili_client,
        }
    }

    pub async fn index_files(
        &self,
    ) -> Result<Vec<FileSystemEntry>, meilisearch_sdk::errors::Error> {
        let mut scanned_entries = Vec::new();

        // Load the ignore rules (e.g., from a .gitignore file or custom rules)
        //let ignore_rules = gitignore::Gitignore::new(&self.directory).unwrap();

        // Recursively scan the directory
        // Use WalkBuilder to apply ignore rules efficiently
        // TODO: maybe record the uuid with modification time and skip ones same as the last-time scan
        let mut walkerbuilder = WalkBuilder::new(&self.project_config.root);
        walkerbuilder
            .standard_filters(false)
            .hidden(!self.project_config.index_hidden)
            .max_depth(self.project_config.max_depth)
            .follow_links(self.project_config.follow_symlinks);

        if let Some(custom_ignore_rule_file) = &self.project_config.custom_ignore_rule_file {
            walkerbuilder.add_custom_ignore_filename(custom_ignore_rule_file);
        }

        for entry in walkerbuilder.build().filter_map(Result::ok) {
            let path = entry.path();

            // Index both files and folders (ignoring based on the rules)
            if let Some(index_entry) = Indexer::entry_to_index(&path) {
                scanned_entries.push(index_entry);
            }
        }

        // send to the client
        // TODO: incrementally sending indexes and deleting obselete indexes smartly
        if let Some(unwrapped_meili_client) = &self.meili_client {
            let meili_index = unwrapped_meili_client.index("filesystem_index");
            // delete old index
            let delete_operation = meili_index.delete_all_documents().await;
            if delete_operation.is_err() {
                eprintln!("Failed to delete old index!");
            }
            // create new index
            let create_operation = meili_index
                .add_documents(&scanned_entries, Some("uuid"))
                .await;
            if create_operation.is_err() {
                eprintln!("Failed to create new index!");
            }
        }
        Ok(scanned_entries)
    }

    fn entry_to_index(path: &Path) -> Option<FileSystemEntry> {
        let metadata = fs::metadata(path).ok()?;
        let name = path.file_name()?.to_string_lossy().to_string();
        let is_hidden = name.starts_with('.');

        let entry_type = if path.is_file() {
            IndexEntryType::File
        } else if path.is_dir() {
            IndexEntryType::Folder
        } else {
            return None; // Skip special files like symlinks
        };

        let size = if entry_type == IndexEntryType::File {
            Some(metadata.len())
        } else {
            None // Folders don't have a meaningful size
        };

        let modified_date = metadata.modified().ok().and_then(|time| {
            let datetime: DateTime<Utc> = time.into();
            Some(datetime)
        });

        let path_str = path.to_string_lossy().to_string();
        let uuid = Uuid::new_v5(&Uuid::NAMESPACE_URL, path_str.as_bytes()).to_string();

        Some(FileSystemEntry {
            uuid,
            path: path_str,
            name,
            entry_type,
            size,
            modified_date,
            is_hidden,
            preview: None, // Only relevant for files
        })
    }
}

pub async fn is_meilisearch_running(meilisearch_config: &MeiliSearchConfig) -> bool {
    let meilisearch_url = &meilisearch_config.meilisearch_url;
    let meilisearch_api_key = &meilisearch_config.meilisearch_api_key;
    let meili_client =
        meilisearch_sdk::client::Client::new(meilisearch_url, Some(meilisearch_api_key));

    if let Ok(meili_client) = meili_client {
        return meili_client.health().await.is_ok();
    } else {
        return false;
    }
}
