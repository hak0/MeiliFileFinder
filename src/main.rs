mod config;
mod file_index;
mod indexer;
mod scheduler;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Read Config   
    let config = config::read_config("config.toml").expect("Failed to read config file");
    println!("Config Loaded!");
    println!("{:}", config);

    // Start scheduler
    println!("Starting Scheduler!");
    scheduler::schedule_projects(&config.projects, &config.meilisearch).await.expect("Failed to initialize scheduler for projects");
}