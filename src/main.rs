mod config;
mod file_index;
mod indexer;
mod scheduler;
mod server;

use tokio::signal;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Read Config
    let config = config::read_config("doc/config.toml").expect("Failed to read config file");
    println!("Config Loaded!");
    println!("{:}", config);

    let server = server::start_server();

    let scheduler = scheduler::schedule_projects(&config.projects, &config.meilisearch);

    // Signal handling for graceful shutdown
    let signal_handler = async {
        signal::ctrl_c()
            .await
            .expect("Failed to listen for CTRL+C");
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
