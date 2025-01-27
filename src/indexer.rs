use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use crate::file_index::{FileSystemEntry, IndexEntryType};

#[cfg(test)]
#[path = "tests/indexer_tests.rs"]
mod file_index_tests;

pub struct Indexer {
    pub directory: PathBuf,
}

impl Indexer {
    // Create a new Indexer instance
    pub fn new(directory: &Path) -> Self {
        Indexer {
            directory: directory.to_path_buf(),
        }
    }

    pub fn index_files(&self) -> Vec<FileSystemEntry> {
        let mut indexed_entries = Vec::new();

        // Recursively scan the directory
        if let Ok(entries) = fs::read_dir(&self.directory) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    // Index both files and folders
                    if let Some(index_entry) = Indexer::entry_to_index(&path) {
                        indexed_entries.push(index_entry);
                    }
                }
            }
        }

        indexed_entries
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

        Some(FileSystemEntry {
            name,
            path: path.to_string_lossy().to_string(),
            entry_type,
            size,
            modified_date,
            is_hidden,
            preview: None, // Only relevant for files
        })
    }
}
