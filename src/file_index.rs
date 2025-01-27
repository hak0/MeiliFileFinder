use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(test)]
#[path = "tests/file_index_tests.rs"]
mod file_index_tests;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum IndexEntryType {
    File,
    Folder,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileSystemEntry {
    pub uuid: String,
    pub path: String,                         // The full path to the file or folder
    pub name: String,                         // The name of the file or folder
    pub entry_type: IndexEntryType,           // Whether it's a file or folder
    pub size: Option<u64>,                    // Size in bytes (None for folders)
    pub modified_date: Option<DateTime<Utc>>, // Last modified timestamp (optional for folders)
    pub is_hidden: bool,                      // Whether the entry is hidden
    pub preview: Option<String>,              // Optional preview content (for files only)
}

