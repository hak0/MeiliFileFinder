use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::config::{MeiliSearchConfig, ProjectConfig};
use crate::indexer;

pub async fn schedule_projects(
    projects: &[ProjectConfig], // Use slice instead of &Vec for better ergonomics
    meilisearch_config: &MeiliSearchConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Scheduler!");
    let sched = JobScheduler::new().await?;

    // one-job at a time, skip the job if it's already running
    let is_running = Arc::new(AtomicBool::new(false));

    for project in projects {
        let crontab = project.crontab.clone();
        // Clone project into an Arc once per iteration
        let project_arc = Arc::new(project.clone());
        // Clone meilisearch_config once per iteration
        let meilisearch_config_arc = Arc::new(meilisearch_config.clone());

        let is_running_clone = is_running.clone();
        let job = Job::new_async(crontab, move |_uuid, _l| {
            // Clone Arcs to move into the async block
            let project = Arc::clone(&project_arc);
            let meilisearch_config = Arc::clone(&meilisearch_config_arc);
            let is_running_clone = is_running_clone.clone();

            Box::pin(async move {
                if is_running_clone.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) == Ok(false) {
                    println!("Job started for {}", project.id);

                    let indexer = indexer::Indexer::new(&project, &meilisearch_config);

                    indexer.configure_meilisearch_index().await;

                    match indexer.index_files().await {
                        Ok((_, files_count)) => {
                            println!("Indexed {} files in {:?}", files_count, project.root)
                        }
                        Err(e) => eprintln!("Error indexing {:?}: {}", project.root, e),
                    }

                    println!("Job finished for {}", project.id);
                    is_running_clone.store(false, Ordering::SeqCst);
                } else {
                    println!("Another Job is already running! skipping {}", project.id);
                }
            })
        })?;

        sched.add(job).await?;
    }

    // Feature 'signal' must be enabled
    sched.shutdown_on_ctrl_c();

    sched.start().await?;

    // Keep the scheduler running
    loop {
        tokio::time::sleep(Duration::from_secs(3600)).await;
    }
}
