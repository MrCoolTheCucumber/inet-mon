//! Provides functionality to login and query the NR5103E router for its cellular WAN status.

use std::time::Duration;

use anyhow::{anyhow, bail, Result};
use base64::prelude::*;
use cookie::Cookie;
use metrics::gauge;
use reqwest::Client;
use serde::Deserialize;
use tokio::{task::JoinHandle, time::interval};

const ADDR: &str = "https://192.168.1.1";

struct SessionId(String);

pub struct Password(String);

impl<T> From<T> for Password
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code, non_snake_case)]
struct WanStatus {
    CELL_Roaming_Enable: bool,
    INTF_Status: String,
    INTF_IMEI: String,
    INTF_Current_Access_Technology: String,
    INTF_Network_In_Use: String,
    INTF_RSSI: i32,
    INTF_Supported_Bands: String,
    INTF_Current_Band: String,
    INTF_Cell_ID: i32,
    INTF_PhyCell_ID: i32,
    INTF_Uplink_Bandwidth: String,
    INTF_Downlink_Bandwidth: String,
    INTF_RFCN: String,
    INTF_RSRP: i32,
    INTF_RSRQ: i32,
    INTF_RSCP: i32,
    INTF_EcNo: i32,
    INTF_TAC: i32,
    INTF_LAC: i32,
    INTF_RAC: i32,
    INTF_BSIC: i32,
    INTF_SINR: i32,
    INTF_CQI: i32,
    INTF_MCS: i32,
    INTF_RI: i32,
    INTF_PMI: i32,
    INTF_Module_Software_Version: String,
    USIM_Status: String,
    USIM_IMSI: String,
    USIM_ICCID: String,
    USIM_PIN_Protection: bool,
    USIM_PIN_Remaining_Attempts: i32,
    Passthru_Enable: bool,
    Passthru_Mode: String,
    Passthru_MacAddr: String,
    NSA_Enable: bool,
    NSA_MCC: String,
    NSA_MNC: String,
    NSA_PhyCellID: i32,
    NSA_RFCN: i32,
    NSA_Band: String,
    NSA_RSSI: i32,
    // NSA_UplinkBandwidth: null,
    // NSA_DownlinkBandwidth: null,
    NSA_RSRP: i32,
    NSA_RSRQ: i32,
    NSA_SINR: i32,
    // SCC_Info: []
}

pub fn start_mon(password: impl Into<Password>) -> JoinHandle<()> {
    let password = password.into();
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Unable to build http client?");

    tokio::spawn(async move {
        let session = login(password, &client).await.unwrap();
        let mut interval = interval(Duration::from_secs(5));

        loop {
            interval.tick().await;

            let wan_status = get_wan_status(&client, &session).await.unwrap();
            handle_metrics(wan_status);
        }
    })
}

async fn login(password: Password, client: &Client) -> Result<SessionId> {
    let url = format!("{}/UserLogin", ADDR);
    let base64_password = BASE64_STANDARD.encode(password.0);

    let body = format!(
        r#"{{"Input_Account":"admin","Input_Passwd":"{}","currLang":"en","RememberPassword":0,"SHA512_password":false}}"#,
        base64_password
    );

    let response = client.post(url).body(body).send().await?;
    let headers = response.headers();

    let Some(raw_cookie) = headers.get("set-cookie") else {
        bail!("Unable to find cookie in auth response");
    };

    let cookie = Cookie::parse(raw_cookie.to_str()?)?;
    Ok(SessionId(cookie.value().to_owned()))
}

async fn get_wan_status(client: &Client, session: &SessionId) -> Result<WanStatus> {
    let wan_status_url = format!("{}/cgi-bin/DAL?oid=cellwan_status", ADDR);

    let response = client
        .get(&wan_status_url)
        .header("Cookie", format!("Session={}", &session.0))
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;
    let wan_status = deser_wan_status_response(json)?;

    Ok(wan_status)
}

fn deser_wan_status_response(response: serde_json::Value) -> Result<WanStatus> {
    let raw = response
        .as_object()
        .and_then(|obj| obj.get("Object"))
        .and_then(|obj| obj.as_array())
        .and_then(|arr| arr.first())
        .ok_or(anyhow!("Unable to parse wam status response"))?;

    let wan_status: WanStatus = serde_json::from_value(raw.clone())?;
    Ok(wan_status)
}

fn handle_metrics(wan_status: WanStatus) {
    gauge!("cellular.up").set(if wan_status.INTF_Status == "Up" {
        1.0
    } else {
        0.0
    });

    gauge!("cellular.RSSI").set(wan_status.INTF_RSSI);
    gauge!("cellular.RSRP").set(wan_status.INTF_RSRP);
    gauge!("cellular.SINR").set(wan_status.INTF_SINR);
    gauge!("cellular.RSRQ").set(wan_status.INTF_RSRQ);
}
