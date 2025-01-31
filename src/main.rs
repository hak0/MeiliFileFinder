mod config;
mod file_index;
mod indexer;
mod scheduler;
mod server;

use std::sync::Arc;
use tokio::process::Command;
use tokio::signal;
use tokio::sync::Mutex;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Read Config
    let config = config::read_config("doc/config.toml").expect("Failed to read config file");
    println!("Config Loaded!\n");
    println!("{:}", config);

    check_and_start_meilisearch(&config.meilisearch).await;

    let server = server::start_server(&config.meilisearch);

    let scheduler = scheduler::schedule_projects(&config.projects, &config.meilisearch);

    // Signal handling for graceful shutdown
    let signal_handler = async {
        signal::ctrl_c().await.expect("Failed to listen for CTRL+C");
        println!("CTRL+C received! Shutting down...");
    };

    // Join the server, scheduler, and signal handler
    tokio::select! {
        _ = async {
            let res = tokio::join!(server, scheduler);
            match res {
                (Ok(_), Ok(_)) => println!("Server and Scheduler exited successfully!"),
                (Err(e), _) => eprintln!("Server failed to start: {:?}", e),
                (_, Err(e)) => eprintln!("Scheduler failed to start: {:?}", e),
            }
        } => {},
        _ = signal_handler => println!(),
    }
}

async fn check_and_start_meilisearch(meilisearch_config: &config::MeiliSearchConfig) {
    // check if meilisearch is started, and start it if necessary

    let meilisearch_child = Arc::new(Mutex::new(None));

    // Check if Meilisearch is running and start if necessary
    if !indexer::is_meilisearch_running(&meilisearch_config).await {
        println!("No available Meilisearch Instance.");

        // parse startup configs
        let meilisearch_bin_path = &meilisearch_config.meilisearch_bin_path;
        if meilisearch_bin_path.is_empty() {
            eprintln!("Meilisearch binary path is empty. Exiting.");
            std::process::exit(1);
        }
        println!("Starting Meilisearch from {}", meilisearch_bin_path);

        let meilisearch_master_key = &meilisearch_config.meilisearch_api_key;
        if meilisearch_master_key.len() < 16 {
            eprintln!("Meilisearch API key needs to be at least 16 bytes. Exiting.");
            std::process::exit(1);
        };

        let meilisearch_url = &meilisearch_config.meilisearch_url;
        let meilisearch_url_no_prefix = meilisearch_url
            .strip_prefix("http://")
            .or_else(|| meilisearch_url.strip_prefix("https://"))
            .unwrap_or(meilisearch_url);
        let meilisearch_db_path = &meilisearch_config.meilisearch_db_path;
        if meilisearch_db_path.is_empty() {
            eprintln!("Meilisearch database path is not set, creating from workingdir")
        }
        let meilisearch_telemetry = meilisearch_config.meilisearch_telemetry;

        // assign environemnt variables for child_builder
        let mut child_builder = Command::new(&meilisearch_bin_path);
        child_builder.kill_on_drop(true);  // Automatically kill Meilisearch on parent exit
        child_builder.env("MEILI_HTTP_ADDR", meilisearch_url_no_prefix);
        child_builder.env("MEILI_MASTER_KEY", meilisearch_master_key);
        if !meilisearch_db_path.is_empty() {
            child_builder.env("MEILI_DB_PATH", meilisearch_db_path);
        }
        if meilisearch_telemetry == false {
            child_builder.arg("--no-analytics");
        }

        // Spawn Meilisearch
        let child = child_builder.spawn().expect("Failed to start Meilisearch");
        *meilisearch_child.lock().await = Some(child);

        // Wait for Meilisearch to become ready
        println!("Waiting for Meilisearch to start...");
        let mut attempts = 0;
        let max_attempts = 30;
        let delay = tokio::time::Duration::from_secs(1);
        loop {
            if indexer::is_meilisearch_running(&meilisearch_config).await {
                println!("Meilisearch is ready!");
                break;
            }
            if attempts >= max_attempts {
                eprintln!("Meilisearch did not start in time. Exiting.");
                std::process::exit(1);
            }
            attempts += 1;
            tokio::time::sleep(delay).await;
        }
    } else {
        println!("Meilisearch is already running.");
    }
}
