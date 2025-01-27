mod file_index;
mod indexer;

use indexer::Indexer;
use std::path::Path;

const MEILISEARCH_URL: &str = "http://localhost:7700";
const MEILISEARCH_API_KEY: &str = "hello_world123456";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Define the directory to index
    let directory = Path::new("./");

    // Create an Indexer instance
    let indexer = Indexer::new(directory, MEILISEARCH_URL, MEILISEARCH_API_KEY);

    // Index the files
    let files = indexer.index_files().await;

    // Print the indexed files
    for file in files {
        println!("{:?}", file);
    }
}