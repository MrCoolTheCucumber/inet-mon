use serde::Deserialize;

pub struct SessionId(pub String);

pub struct Password(pub(super) String);

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
pub struct WanStatus {
    pub CELL_Roaming_Enable: bool,
    pub INTF_Status: String,
    pub INTF_IMEI: String,
    pub INTF_Current_Access_Technology: String,
    pub INTF_Network_In_Use: String,
    pub INTF_RSSI: i32,
    pub INTF_Supported_Bands: String,
    pub INTF_Current_Band: String,
    pub INTF_Cell_ID: i32,
    pub INTF_PhyCell_ID: i32,
    pub INTF_Uplink_Bandwidth: String,
    pub INTF_Downlink_Bandwidth: String,
    pub INTF_RFCN: String,
    pub INTF_RSRP: i32,
    pub INTF_RSRQ: i32,
    pub INTF_RSCP: i32,
    pub INTF_EcNo: i32,
    pub INTF_TAC: i32,
    pub INTF_LAC: i32,
    pub INTF_RAC: i32,
    pub INTF_BSIC: i32,
    pub INTF_SINR: i32,
    pub INTF_CQI: i32,
    pub INTF_MCS: i32,
    pub INTF_RI: i32,
    pub INTF_PMI: i32,
    pub INTF_Module_Software_Version: String,
    pub USIM_Status: String,
    pub USIM_IMSI: String,
    pub USIM_ICCID: String,
    pub USIM_PIN_Protection: bool,
    pub USIM_PIN_Remaining_Attempts: i32,
    pub Passthru_Enable: bool,
    pub Passthru_Mode: String,
    pub Passthru_MacAddr: String,
    pub NSA_Enable: bool,
    pub NSA_MCC: String,
    pub NSA_MNC: String,
    pub NSA_PhyCellID: i32,
    pub NSA_RFCN: i32,
    pub NSA_Band: String,
    pub NSA_RSSI: i32,
    // NSA_UplinkBandwidth: null,
    // NSA_DownlinkBandwidth: null,
    pub NSA_RSRP: i32,
    pub NSA_RSRQ: i32,
    pub NSA_SINR: i32,
    // SCC_Info: []
}
