use crate::config::{MeiliSearchConfig, ProjectConfig};
use crate::file_index::IndexEntryType;
use crate::indexer::Indexer;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::tempdir;


// create a mock meilisearch client that always return ok but do nothing

fn generate_test_config(rootpath: &Path) -> (MeiliSearchConfig, ProjectConfig) {
    let meilisearch_config = MeiliSearchConfig {
        meilisearch_url: "dummy_url".to_string(),
        meilisearch_api_key: "dummy_key".to_string(),
        meilisearch_bin_path: "".to_string(),
        meilisearch_db_path: "".to_string(),
        meilisearch_telemetry: true,
    };
    let project_config = ProjectConfig {
        root: PathBuf::from(rootpath),
        crontab: "".to_string(),
        index_hidden: true,
        max_depth: 0,
        follow_symlinks: false,
        custom_ignore_rule_file: None,
    };
    (meilisearch_config, project_config)
}

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
    let (meilisaerch_config, project_config) = generate_test_config(dir_path);
    let mut indexer = Indexer::new(&project_config, &meilisaerch_config);
    indexer.meili_client = None;

    // Perform indexing
    let entries = indexer.index_files().await.unwrap();

    // Validate results
    // 1 child folder, 1 child file and 1 parent
    assert_eq!(entries.len(), 3);

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
    let (meilisaerch_config, project_config) = generate_test_config(dir_path);
    let mut indexer = Indexer::new(&project_config, &meilisaerch_config);
    indexer.meili_client = None;

    // Perform indexing
    let entries = indexer.index_files().await.unwrap();

    // Validate the hidden file is indexed
    assert_eq!(entries.len(), 2);
    let hidden_entry = entries.iter().find(|e| e.name == ".hidden_file").unwrap();
    assert_eq!(hidden_entry.entry_type, IndexEntryType::File);
    assert!(!hidden_entry.size.is_none());
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
    let (meilisaerch_config, project_config) = generate_test_config(dir_path);
    let mut indexer = Indexer::new(&project_config, &meilisaerch_config);
    indexer.meili_client = None;

    // Perform indexing
    let entries = indexer.index_files().await.unwrap();

    // Validate the hidden folder is indexed
    // 1 parent and 1 child hidden folder
    assert_eq!(entries.len(), 2);
    let hidden_entry = entries.iter().find(|e| e.name == ".hidden_folder").unwrap();
    assert_eq!(hidden_entry.entry_type, IndexEntryType::Folder);
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
    let (meilisaerch_config, project_config) = generate_test_config(dir_path);
    let mut indexer = Indexer::new(&project_config, &meilisaerch_config);
    indexer.meili_client = None;

    // Perform indexing
    let entries = indexer.index_files().await.unwrap();

    // Validate results
    assert_eq!(entries.len(), 6);

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
