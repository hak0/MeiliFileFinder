use crate::config::{MeiliSearchConfig, ProjectConfig};
use crate::file_index::{FileSystemEntry, IndexEntryType};
use chrono::{DateTime, Utc};
use ignore::WalkBuilder;
use meilisearch_sdk::documents::DocumentDeletionQuery;
use meilisearch_sdk::indexes::IndexesQuery;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[cfg(test)]
#[path = "tests/indexer_tests.rs"]
mod file_index_tests;

#[derive(Debug, Clone)]
pub struct Indexer {
    pub project_config: ProjectConfig,
    pub meili_index_name: String,
    pub meili_client: Option<meilisearch_sdk::client::Client>,
}

// suppose each entry take 2kb, 10000 entries will take 20MB,
// which is lower than the default meilisearch batch limit of 100MB
const MEILISEARCH_BATCH_ENTRIES_LIMIT: usize = 10000;

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
            meili_index_name: meilisearch_config.meilisearch_index_name.clone(),
            meili_client,
        }
    }

    pub async fn configure_meilisearch_index(&self) {
        let index_name = &self.meili_index_name;
        if let Some(unwrapped_meili_client) = &self.meili_client {
            // List all indexes
            let indexes_query = IndexesQuery::new(&unwrapped_meili_client)
                .with_limit(1024)
                .execute()
                .await;
            if indexes_query.is_err() {
                eprintln!("Failed to list indexs!");
            }
            let meili_indexes: HashSet<String> = indexes_query
                .unwrap()
                .results
                .into_iter()
                .map(|index| index.uid)
                .collect();

            // Create index if not existing
            if !meili_indexes.contains(index_name) {
                let index_creation = unwrapped_meili_client
                    .create_index(index_name, Some("uuid"))
                    .await;
                if index_creation.is_err() {
                    eprintln!("Failed to create index {}!", index_name);
                }
            }

            // Update filterable attributes
            let existing_filterable_attributes: HashSet<String> = match unwrapped_meili_client
                .index(index_name)
                .get_filterable_attributes()
                .await
            {
                Ok(filterable_attributes) => filterable_attributes.into_iter().collect(),
                Err(_) => HashSet::new(),
            };
            let filterable_attributes = [
                "path",
                "name",
                "entry_type",
                "size",
                "modified_date",
                "is_hidden",
                "preview",
                "project_id",
                "entry_last_updated",
            ];
            if filterable_attributes
                .iter()
                .any(|attr| !existing_filterable_attributes.contains(*attr))
            {
                let filterable_attributes = unwrapped_meili_client
                    .index(index_name)
                    .set_filterable_attributes(&filterable_attributes)
                    .await;
                if filterable_attributes.is_err() {
                    eprintln!("Failed to update filterable attributes!");
                }
            }

            // Update sortable attributes
            let existing_sortable_attibutes: HashSet<String> = match unwrapped_meili_client
                .index(index_name)
                .get_sortable_attributes()
                .await
            {
                Ok(sortable_attributes) => sortable_attributes.into_iter().collect(),
                Err(_) => HashSet::new(),
            };
            let sortable_attributes = ["path", "name", "size", "modified_date"];
            if sortable_attributes
                .iter()
                .any(|attr| !existing_sortable_attibutes.contains(*attr))
            {
                let sortable_attributes = unwrapped_meili_client
                    .index(index_name)
                    .set_sortable_attributes(&sortable_attributes)
                    .await;
                if sortable_attributes.is_err() {
                    eprintln!("Failed to update sortable attributes!");
                }
            }

            // Update separators to preserve symbol and numbers in the path, etc.
            let seperators_to_remove = [
                ".", "/", "\\", "@", "#", "$", "%", "^", "&", "*", "(", ")", "-", "_", "+", "=", " ",
            ];
            let update_seperators = unwrapped_meili_client
                .index(index_name)
                // .set_dictionary(["@", sep])
                .set_non_separator_tokens(&seperators_to_remove.into_iter().map(|s| s.to_string()).collect())
                .await;
            if update_seperators.is_err() {
                eprintln!("Failed to update separators!");
            }
        }
    }

    pub async fn index_files(
        &self,
    ) -> Result<(Vec<FileSystemEntry>, usize), meilisearch_sdk::errors::Error> {
        let mut scanned_entries = Vec::new();
        // scan and index files and folders
        // return the last batch of scanned entries and the total size of all scanned entries

        // Recursively scan the directory
        // Use WalkBuilder to apply ignore rules efficiently
        // TODO: maybe record the uuid with modification time and skip ones same as the last-time scan
        let mut walkerbuilder = WalkBuilder::new(&self.project_config.root);
        walkerbuilder
            .standard_filters(false)
            .hidden(!self.project_config.index_hidden)
            .follow_links(self.project_config.follow_symlinks);

        if self.project_config.max_depth > 0 {
            walkerbuilder.max_depth(Some(self.project_config.max_depth));
        } else {
            walkerbuilder.max_depth(None);
        }

        if let Some(custom_ignore_rule_file) = &self.project_config.custom_ignore_rule_file {
            walkerbuilder.add_custom_ignore_filename(custom_ignore_rule_file);
        }

        let time_now = Utc::now();

        let mut scanned_entries_total_count = 0;
        for entry in walkerbuilder.build().filter_map(Result::ok) {
            let path = entry.path();

            // Index both files and folders (ignoring based on the rules)
            if let Some(index_entry) = self.entry_to_index(&path, &time_now).await {
                scanned_entries.push(index_entry);
                scanned_entries_total_count += 1;
            }

            // Send the batch of 10000 entries to MeiliSearch
            if scanned_entries.len() >= MEILISEARCH_BATCH_ENTRIES_LIMIT {
                self.send_entries_to_meilisearch(&scanned_entries).await;
                scanned_entries.clear();
            }
        }

        // Send remaining entries to MeiliSearch
        self.send_entries_to_meilisearch(&scanned_entries).await;

        // Clean obselete index
        self.clean_obselete_index(&time_now).await;

        Ok((scanned_entries, scanned_entries_total_count))
    }

    async fn clean_obselete_index(&self, update_time: &DateTime<Utc>) {
        if let Some(unwrapped_meili_client) = &self.meili_client {
            let meili_index = unwrapped_meili_client.index(&self.meili_index_name);
            let delete_operation = DocumentDeletionQuery::new(&meili_index)
                .with_filter(&format!(
                    "(project_id = {}) AND (entry_last_updated < {})",
                    self.project_config.id,
                    update_time.timestamp()
                ))
                .execute::<()>()
                .await;
            if delete_operation.is_err() {
                eprintln!("Failed to delete old index!");
            }
        }
    }

    async fn send_entries_to_meilisearch(&self, scanned_entries: &Vec<FileSystemEntry>) {
        if let Some(unwrapped_meili_client) = &self.meili_client {
            let meili_index = unwrapped_meili_client.index(&self.meili_index_name);
            let create_operation = meili_index.add_documents(scanned_entries, None).await;
            if create_operation.is_err() {
                eprintln!("Failed to create new index!");
                for entry in scanned_entries {
                    eprintln!("  Failed to index: {:?}", entry.path);
                }
            }
        }
    }

    async fn entry_to_index(
        &self,
        path: &Path,
        update_time: &DateTime<Utc>,
    ) -> Option<FileSystemEntry> {
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
            project_id: self.project_config.id.clone(),
            entry_last_updated: update_time.timestamp(),
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
