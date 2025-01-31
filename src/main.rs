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

    let meilisearch_child= check_and_start_meilisearch(&config.meilisearch).await;

    let server = server::start_server(&config.meilisearch);

    let scheduler = scheduler::schedule_projects(&config.projects, &config.meilisearch);

    // Join the server, scheduler, and signal handler
    tokio::select! {
        // Join server and scheduler tasks
        _ = async {
            let res = tokio::join!(server, scheduler);
            match res {
                (Ok(_), Ok(_)) => println!("Server and Scheduler exited successfully!"),
                (Err(e), _) => eprintln!("Server failed to start: {:?}", e),
                (_, Err(e)) => eprintln!("Scheduler failed to start: {:?}", e),
            }
        } => {},
        // Signal Handler
        _ = unified_signal_handler(meilisearch_child) => println!(),
    }
}

async fn check_and_start_meilisearch(
    meilisearch_config: &config::MeiliSearchConfig,
) -> Option<Arc<Mutex<tokio::process::Child>>> {
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
        child_builder.kill_on_drop(true); // Automatically kill Meilisearch on parent exit
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
        let meilisearch_child = Arc::new(Mutex::new(child));

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
        Some(meilisearch_child)
    } else {
        println!("Meilisearch is already running.");
        None
    }
}

async fn unified_signal_handler(meilisearch_child: Option<Arc<Mutex<tokio::process::Child>>>) {
    // Handle cross-platform `CTRL+C`
    let ctrl_c = async {
        signal::ctrl_c().await.expect("Failed to listen for CTRL+C");
        println!("CTRL+C received. Terminating child process...");
    };

    #[cfg(unix)]
    let unix_signals = async {
        // Register Unix-specific signals
        let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to register SIGTERM handler");
        let mut sigint = signal::unix::signal(signal::unix::SignalKind::interrupt())
            .expect("Failed to register SIGINT handler");
        let mut sighup = signal::unix::signal(signal::unix::SignalKind::hangup())
            .expect("Failed to register SIGHUP handler");

        tokio::select! {
            _ = sigterm.recv() => println!("SIGTERM received. Terminating child process..."),
            _ = sigint.recv() => println!("SIGINT received. Terminating child process..."),
            _ = sighup.recv() => println!("SIGHUP received. Terminating child process..."),
        }
    };

    #[cfg(not(unix))]
    let unix_signals = async {
        // No-op on non-Unix platforms
        futures::future::pending::<()>().await;
    };

    // Use `tokio::select!` to wait for any signal
    tokio::select! {
        _ = ctrl_c => {},
        _ = unix_signals => {},
    }

    // Kill the child process if running
    if let Some(child_mutex) = meilisearch_child {
        let mut child = child_mutex.lock().await;
        match child.kill().await {
            Ok(_) => println!("Child process terminated successfully."),
            Err(e) => eprintln!("Failed to terminate child process: {:?}", e),
        }
    }

    // Exit the program
    std::process::exit(0);
}