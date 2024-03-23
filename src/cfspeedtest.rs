//! Provides programatic functionality for the `cfspeedtest` cli tool

use std::time::Duration;

use anyhow::Result;
use cfspeedtest::{speedtest::run_latency_test, OutputFormat};
use metrics::{gauge, histogram};
use tokio::{task::JoinHandle, time::interval};

pub fn start_mon() -> JoinHandle<()> {
    tokio::spawn(async {
        let mut interval = interval(Duration::from_secs(60 * 5));

        // de-sync from speedtestnet tests
        tokio::time::sleep(Duration::from_secs(60)).await;

        loop {
            interval.tick().await;

            let _ = tokio::task::spawn_blocking(|| {
                if let Err(e) = run_speed_test() {
                    tracing::error!(?e, "cfspeedtest failed");
                }
            })
            .await;
        }
    })
}

fn run_speed_test() -> Result<()> {
    let (latency_results, avg) = run_latency_test(
        &reqwest::blocking::Client::new(),
        25,
        OutputFormat::None, // don't write to stdout while running the test
    );

    gauge!("cfspeedtest.avg_latency").set(avg);

    for result in latency_results {
        histogram!("cfspeedtest.latency").record(result);
    }

    Ok(())
}
