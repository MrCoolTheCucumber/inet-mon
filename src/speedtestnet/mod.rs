//! Provides programatic functionality for the `cfspeedtest` cli tool

use std::time::Duration;

use anyhow::{anyhow, Result};
use async_process::{Command, Stdio};
use futures::{io::BufReader, AsyncBufReadExt as _, StreamExt};
use metrics::gauge;
use tokio::{task::JoinHandle, time::interval};

use crate::speedtestnet::types::SpeedTestResult;

mod types;

const FIVE_MINS: Duration = Duration::from_secs(60 * 5);
const HYPEROPTIC_SERVER_ID: &str = "14679";

pub fn start_mon() -> JoinHandle<()> {
    tokio::spawn(async {
        let mut interval = interval(FIVE_MINS);

        loop {
            interval.tick().await;

            if let Err(e) = run_speed_test().await {
                tracing::error!(?e, "speedtestnet failed");
            }
        }
    })
}

async fn run_speed_test() -> Result<()> {
    let mut child = Command::new("speedtest")
        .args(["-s", HYPEROPTIC_SERVER_ID, "-f", "json"])
        .stderr(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

    while let Some(line) = lines.next().await {
        let line = line?;
        let json: serde_json::Value = serde_json::from_str(&line)?;

        let obj = json
            .as_object()
            .ok_or(anyhow!("Could not parse json blob as object"))?;
        let r#type = obj
            .get("type")
            .and_then(|result| result.as_str())
            .ok_or(anyhow!("json blob had no type field?"))?;

        if r#type == "result" {
            handle_result(json)?;
            break;
        }
    }

    Ok(())
}

fn handle_result(json: serde_json::Value) -> Result<()> {
    let speedtest_results = serde_json::from_value::<SpeedTestResult>(json.clone())?;

    let download_bandwidth = speedtest_results.download.bandwidth as f64 / 175_000.0;
    let upload_bandwidth = speedtest_results.upload.bandwidth as f64 / 175_000.0;

    gauge!("speedtestnet.bandwidth", "type" => "download").set(download_bandwidth);
    gauge!("speedtestnet.bandwidth", "type" => "upload").set(upload_bandwidth);

    gauge!("speedtestnet.ping", "type" => "high").set(speedtest_results.ping.high);
    gauge!("speedtestnet.ping", "type" => "jitter").set(speedtest_results.ping.jitter);
    gauge!("speedtestnet.ping", "type" => "low").set(speedtest_results.ping.low);
    gauge!("speedtestnet.ping", "type" => "latency").set(speedtest_results.ping.latency);

    gauge!("speedtestnet.packet_loss").set(speedtest_results.packet_loss);

    Ok(())
}
