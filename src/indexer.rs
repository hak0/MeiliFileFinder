use crate::file_index::{FileSystemEntry, IndexEntryType};
use chrono::{DateTime, Utc};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[cfg(test)]
#[path = "tests/indexer_tests.rs"]
mod file_index_tests;

pub struct Indexer {
    pub directory: PathBuf,
    pub meili_client: Option<meilisearch_sdk::client::Client>,
}

impl Indexer {
    // Create a new Indexer instance
    pub fn new(directory: &Path, meilisearch_url: &str, meilisearch_api_key: &str) -> Self {
        let meili_client = if !meilisearch_url.is_empty() && !meilisearch_api_key.is_empty() {
            meilisearch_sdk::client::Client::new(meilisearch_url, Some(meilisearch_api_key)).ok()
        } else {
            None
        };

        Indexer {
            directory: directory.to_path_buf(),
            meili_client,
        }
    }

    pub async fn index_files(&self) -> Vec<FileSystemEntry> {
        let mut scanned_entries = Vec::new();

        // Recursively scan the directory
        if let Ok(entries) = fs::read_dir(&self.directory) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    // Index both files and folders
                    if let Some(index_entry) = Indexer::entry_to_index(&path) {
                        scanned_entries.push(index_entry);
                    }
                }
            }
        }

        // send to the client
        if let Some(unwrapped_meili_client) = &self.meili_client {
            let meili_index = unwrapped_meili_client.index("filesystem_index");
            meili_index
                .add_documents(&scanned_entries, Some("uuid"))
                .await
                .unwrap();
        }
        scanned_entries
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
