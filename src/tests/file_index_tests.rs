use crate::file_index::{FileSystemEntry, IndexEntryType};
use chrono::Utc;
use uuid::Uuid;

#[test]
fn test_filesystem_entry_file() {
    let entry = FileSystemEntry {
        uuid: Uuid::new_v5(
            &Uuid::NAMESPACE_URL,
            "/some/path/file1.txt".to_string().as_bytes(),
        )
        .to_string(),
        path: "/some/path/file1.txt".to_string(),
        name: "file1.txt".to_string(),
        entry_type: IndexEntryType::File,
        size: Some(1024),
        modified_date: Some(Utc::now()),
        is_hidden: false,
        preview: Some("This is a preview".to_string()),
        project_id: "project1".to_string(),
        entry_last_updated: 0,
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
        uuid: Uuid::new_v5(
            &Uuid::NAMESPACE_URL,
            "/some/path/my_folder".to_string().as_bytes(),
        )
        .to_string(),
        name: "my_folder".to_string(),
        path: "/some/path/my_folder".to_string(),
        entry_type: IndexEntryType::Folder,
        size: None,
        modified_date: None,
        is_hidden: false,
        preview: None,
        project_id: "project1".to_string(),
        entry_last_updated: 0,
    };

    assert_eq!(entry.name, "my_folder");
    assert_eq!(entry.entry_type, IndexEntryType::Folder);
    assert!(entry.size.is_none());
    assert!(entry.preview.is_none());
}

#[test]
fn test_filesystem_entry_hidden() {
    let entry = FileSystemEntry {
        uuid: Uuid::new_v5(
            &Uuid::NAMESPACE_URL,
            "/some/path/.hidden_folder".to_string().as_bytes(),
        )
        .to_string(),
        name: ".hidden_folder".to_string(),
        path: "/some/path/.hidden_folder".to_string(),
        entry_type: IndexEntryType::Folder,
        size: None,
        modified_date: None,
        is_hidden: true,
        preview: None,
        project_id: "project1".to_string(),
        entry_last_updated: 0,
    };

    assert_eq!(entry.name, ".hidden_folder");
    assert_eq!(entry.entry_type, IndexEntryType::Folder);
    assert!(entry.is_hidden);
}