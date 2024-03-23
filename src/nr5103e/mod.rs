//! Provides functionality to login and query the NR5103E router for its cellular WAN status.

use std::time::Duration;

use anyhow::{anyhow, bail, Result};
use base64::prelude::*;
use cookie::Cookie;
use metrics::gauge;
use reqwest::Client;
use tokio::{task::JoinHandle, time::interval};

pub use self::types::Password;
use self::types::{SessionId, WanStatus};

mod types;

const ADDR: &str = "https://192.168.1.1";

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

    gauge!("cellular.RSSI", "type" => "INTF").set(wan_status.INTF_RSSI);
    gauge!("cellular.RSRP", "type" => "INTF").set(wan_status.INTF_RSRP);
    gauge!("cellular.SINR", "type" => "INTF").set(wan_status.INTF_SINR);
    gauge!("cellular.RSRQ", "type" => "INTF").set(wan_status.INTF_RSRQ);

    gauge!("cellular.RSSI", "type" => "NSA").set(wan_status.NSA_RSSI);
    gauge!("cellular.RSRP", "type" => "NSA").set(wan_status.NSA_RSRP);
    gauge!("cellular.SINR", "type" => "NSA").set(wan_status.NSA_SINR);
    gauge!("cellular.RSRQ", "type" => "NSA").set(wan_status.NSA_RSRQ);
}
