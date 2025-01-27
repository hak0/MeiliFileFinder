use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;
use crate::indexer::Indexer;
use crate::file_index::IndexEntryType;

#[test]
fn test_index_files_with_files_and_folders() {
    // Create a temporary directory for testing
    let dir = tempdir().unwrap();
    let dir_path = dir.path();

    // Create a file
    let file_path = dir_path.join("file1.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "This is a test file").unwrap();

    // Create a folder
    let folder_path = dir_path.join("folder1");
    fs::create_dir(&folder_path).unwrap();

    // Create the Indexer
    let indexer = Indexer::new(dir_path);

    // Perform indexing
    let entries = indexer.index_files();

    // Validate results
    assert_eq!(entries.len(), 2);

    let file_entry = entries.iter().find(|e| e.name == "file1.txt").unwrap();
    assert_eq!(file_entry.entry_type, IndexEntryType::File);
    assert_eq!(file_entry.size, Some(file_path.metadata().unwrap().len()));
    assert!(!file_entry.is_hidden);

    let folder_entry = entries.iter().find(|e| e.name == "folder1").unwrap();
    assert_eq!(folder_entry.entry_type, IndexEntryType::Folder);
    assert!(folder_entry.size.is_none());
    assert!(!folder_entry.is_hidden);
}

#[test]
fn test_index_hidden_file() {
    // Create a temporary directory for testing
    let dir = tempdir().unwrap();
    let dir_path = dir.path();

    // Create a hidden file
    let hidden_file_path = dir_path.join(".hidden_file");
    File::create(&hidden_file_path).unwrap();

    // Create the Indexer
    let indexer = Indexer::new(dir_path);

    // Perform indexing
    let entries = indexer.index_files();

    // Validate the hidden file is indexed
    assert_eq!(entries.len(), 1);
    let hidden_entry = &entries[0];
    assert_eq!(hidden_entry.name, ".hidden_file");
    assert!(hidden_entry.is_hidden);
}
