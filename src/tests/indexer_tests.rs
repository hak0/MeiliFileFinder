use crate::file_index::IndexEntryType;
use crate::indexer::Indexer;
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;

#[tokio::test]
async fn test_index_files_with_files_and_folders() {
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
    let indexer = Indexer::new(dir_path, "", "");

    // Perform indexing
    let entries = indexer.index_files().await;

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

#[tokio::test]
async fn test_index_hidden_file() {
    // Create a temporary directory for testing
    let dir = tempdir().unwrap();
    let dir_path = dir.path();

    // Create a hidden file
    let hidden_file_path = dir_path.join(".hidden_file");
    File::create(&hidden_file_path).unwrap();

    // Create the Indexer
    let indexer = Indexer::new(dir_path, "", "");

    // Perform indexing
    let entries = indexer.index_files().await;

    // Validate the hidden file is indexed
    assert_eq!(entries.len(), 1);
    let hidden_entry = &entries[0];
    assert_eq!(hidden_entry.name, ".hidden_file");
    assert!(hidden_entry.is_hidden);
}

#[tokio::test]
async fn test_index_hidden_folder() {
    // Create a temporary directory for testing
    let dir = tempdir().unwrap();
    let dir_path = dir.path();

    // Create a hidden folder
    let hidden_folder_path = dir_path.join(".hidden_folder");
    fs::create_dir(&hidden_folder_path).unwrap();

    // Create the Indexer
    let indexer = Indexer::new(dir_path, "", "");

    // Perform indexing
    let entries = indexer.index_files().await;

    // Validate the hidden folder is indexed
    assert_eq!(entries.len(), 1);
    let hidden_entry = &entries[0];
    assert_eq!(hidden_entry.name, ".hidden_folder");
    assert!(hidden_entry.is_hidden);
}

#[tokio::test]
async fn test_index_files_with_nested_subfolders() {
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

    // Create a file in the folder
    let file_path2 = folder_path.join("file2.txt");
    let mut file2 = File::create(&file_path2).unwrap();
    writeln!(file2, "This is another test file").unwrap();

    // Create a subfolder
    let subfolder_path = folder_path.join("subfolder1");
    fs::create_dir(&subfolder_path).unwrap();

    // Create a file in the subfolder
    let file_path3 = subfolder_path.join("file3.txt");
    let mut file3 = File::create(&file_path3).unwrap();
    writeln!(file3, "This is yet another test file").unwrap();

    // Create the Indexer
    let indexer = Indexer::new(dir_path, "", "");

    // Perform indexing
    let entries = indexer.index_files().await;

    // Validate results
    assert_eq!(entries.len(), 4);

    let file_entry = entries.iter().find(|e| e.name == "file1.txt").unwrap();
    assert_eq!(file_entry.entry_type, IndexEntryType::File);
    assert_eq!(file_entry.size, Some(file_path.metadata().unwrap().len()));
    assert!(!file_entry.is_hidden);

    let folder_entry = entries.iter().find(|e| e.name == "folder1").unwrap();
    assert_eq!(folder_entry.entry_type, IndexEntryType::Folder);
    assert!(folder_entry.size.is_none());
    assert!(!folder_entry.is_hidden);

    let file_entry2 = entries.iter().find(|e| e.name == "file2.txt").unwrap();
    assert_eq!(file_entry2.entry_type, IndexEntryType::File);
    assert_eq!(file_entry2.size, Some(file_path2.metadata().unwrap().len()));
    assert!(!file_entry2.is_hidden);

    let file_entry3 = entries.iter().find(|e| e.name == "file3.txt").unwrap();
    assert_eq!(file_entry3.entry_type, IndexEntryType::File);
    assert_eq!(file_entry3.size, Some(file_path3.metadata().unwrap().len()));
    assert!(!file_entry3.is_hidden);
}