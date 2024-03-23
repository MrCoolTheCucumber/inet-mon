use std::env;

use anyhow::Result;
use metrics_exporter_prometheus::PrometheusBuilder;
use tokio::signal;

mod cfspeedtest;
mod nr5103e;
mod speedtestnet;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    PrometheusBuilder::install(PrometheusBuilder::new())?;

    let password = env::var("NR5103E_PASSWD")?;

    let _speedtest_handle = cfspeedtest::start_mon();
    let _nr5103e_handle = nr5103e::start_mon(password);
    let _speedtestnet_handle = speedtestnet::start_mon();

    match signal::ctrl_c().await {
        Ok(_) => tracing::info!("Shutting down..."),
        Err(e) => tracing::error!(
            ?e,
            "An error occured trying to listen for the shutdown signal"
        ),
    }

    Ok(())
}
