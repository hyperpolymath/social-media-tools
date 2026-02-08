use anyhow::Result;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};

use crate::{db, platforms, AppState};

pub async fn start_scheduler(state: AppState) -> Result<JobScheduler> {
    let scheduler = JobScheduler::new().await?;

    // Schedule platform collection jobs
    let collection_job = Job::new_async("0 */15 * * * *", move |_uuid, _l| {
        let state_clone = state.clone();
        Box::pin(async move {
            if let Err(e) = run_collection_cycle(&state_clone).await {
                error!("Collection cycle failed: {}", e);
            }
        })
    })?;

    scheduler.add(collection_job).await?;

    // Start the scheduler
    scheduler.start().await?;

    info!("Scheduler started - will run collection every 15 minutes");

    Ok(scheduler)
}

async fn run_collection_cycle(state: &AppState) -> Result<()> {
    info!("Starting scheduled collection cycle");

    let platforms = db::get_active_platforms(&state.db).await?;
    info!("Found {} active platforms to collect", platforms.len());

    let mut handles = Vec::new();

    for platform in platforms {
        let state_clone = state.clone();
        let platform_clone = platform.clone();

        let handle = tokio::spawn(async move {
            match platforms::collect_platform_policies(&state_clone, &platform_clone).await {
                Ok(results) => {
                    info!(
                        "Collected {} documents for {}",
                        results.len(),
                        platform_clone.name
                    );
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to collect {}: {}", platform_clone.name, e);
                    Err(e)
                }
            }
        });

        handles.push(handle);

        // Respect max concurrent collections
        if handles.len() >= state.config.collector.max_concurrent_collections {
            for handle in handles.drain(..) {
                let _ = handle.await;
            }
        }
    }

    // Wait for remaining jobs
    for handle in handles {
        let _ = handle.await;
    }

    info!("Collection cycle completed");
    Ok(())
}
