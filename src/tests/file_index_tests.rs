use chrono::Utc;
use crate::file_index::{FileSystemEntry, IndexEntryType};

#[test]
fn test_filesystem_entry_file() {
    let entry = FileSystemEntry {
        name: "file1.txt".to_string(),
        path: "/some/path/file1.txt".to_string(),
        entry_type: IndexEntryType::File,
        size: Some(1024),
        modified_date: Some(Utc::now()),
        is_hidden: false,
        preview: Some("This is a preview".to_string()),
    };

    assert_eq!(entry.name, "file1.txt");
    assert_eq!(entry.entry_type, IndexEntryType::File);
    assert_eq!(entry.size, Some(1024));
    assert!(!entry.is_hidden);
    assert!(entry.preview.is_some());
}

#[test]
fn test_filesystem_entry_folder() {
    let entry = FileSystemEntry {
        name: "my_folder".to_string(),
        path: "/some/path/my_folder".to_string(),
        entry_type: IndexEntryType::Folder,
        size: None,
        modified_date: None,
        is_hidden: false,
        preview: None,
    };

    assert_eq!(entry.name, "my_folder");
    assert_eq!(entry.entry_type, IndexEntryType::Folder);
    assert!(entry.size.is_none());
    assert!(entry.preview.is_none());
}
