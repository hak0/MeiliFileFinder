mod file_index;
mod indexer;

use indexer::Indexer;
use std::path::Path;

fn main() {
    // Define the directory to index
    let directory = Path::new("./");

    // Create an Indexer instance
    let indexer = Indexer::new(directory);

    // Index the files
    let files = indexer.index_files();

    // Print the indexed files
    for file in files {
        println!("{:?}", file);
    }
}