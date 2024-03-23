#![allow(unused)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SpeedTestResult {
    pub download: DownloadResults,
    #[serde(rename = "packetLoss", default = "default_packet_loss")]
    pub packet_loss: f64,
    pub ping: PingResults,
    pub upload: UploadResults,
}

fn default_packet_loss() -> f64 {
    0.0
}

#[derive(Debug, Deserialize)]
pub struct DownloadResults {
    pub bandwidth: u64,
    pub bytes: u64,
    pub elapsed: u64,
    pub latency: DownloadLatencyResults,
}

#[derive(Debug, Deserialize)]
pub struct DownloadLatencyResults {
    pub high: f64,
    pub iqm: f64,
    pub jitter: f64,
    pub low: f64,
}

#[derive(Debug, Deserialize)]
pub struct PingResults {
    pub high: f64,
    pub jitter: f64,
    pub latency: f64,
    pub low: f64,
}

#[derive(Debug, Deserialize)]
pub struct UploadResults {
    pub bandwidth: u64,
    pub bytes: u64,
    pub elapsed: u64,
    pub latency: UploadLatencyResults,
}

#[derive(Debug, Deserialize)]
pub struct UploadLatencyResults {
    pub high: f64,
    pub iqm: f64,
    pub jitter: f64,
    pub low: f64,
}
