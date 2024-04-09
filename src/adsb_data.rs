use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub now: f64,
    pub messages: u64,
    pub aircraft: Vec<Aircraft>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aircraft {
    pub hex: String,

    #[serde(rename = "type")]
    pub type_field: String,

    pub flight: Option<String>,

    #[serde(rename = "r")]
    pub registration: Option<String>,

    #[serde(rename = "t")]
    pub aircraft_type: Option<String>,

    #[serde(rename = "alt_baro")]
    pub barometric_altitude: Option<Value>,

    #[serde(rename = "alt_geom")]
    pub geometric_altitude: Option<i64>,

    #[serde(rename = "gs")]
    pub ground_speed: Option<f64>,

    pub track: Option<f64>,

    #[serde(rename = "geom_rate")]
    pub geom_rate: Option<i64>,

    pub category: Option<String>,

    #[serde(rename = "nav_qnh")]
    pub nav_qnh: Option<f64>,

    #[serde(rename = "nav_altitude_mcp")]
    pub nav_altitude_mcp: Option<i64>,

    #[serde(rename = "nav_heading")]
    pub nav_heading: Option<f64>,

    #[serde(rename = "nav_modes")]
    #[serde(default)]
    pub nav_modes: Vec<String>,

    #[serde(rename = "lat")]
    pub latitude: Option<f64>,

    #[serde(rename = "lon")]
    pub longitude: Option<f64>,

    pub nic: Option<i64>,
    pub rc: Option<i64>,

    #[serde(rename = "seen_pos")]
    pub seen_pos: Option<f64>,

    pub version: Option<i64>,

    #[serde(rename = "nic_baro")]
    pub nic_baro: Option<i64>,

    #[serde(rename = "nac_p")]
    pub nac_p: Option<i64>,

    #[serde(rename = "nac_v")]
    pub nac_v: Option<i64>,

    pub sil: Option<i64>,

    #[serde(rename = "sil_type")]
    pub sil_type: Option<String>,

    pub gva: Option<i64>,
    pub sda: Option<i64>,
    pub alert: Option<i64>,
    pub spi: Option<i64>,
    pub mlat: Vec<String>,
    pub tisb: Vec<String>,
    pub messages: i64,
    pub seen: f64,
    pub rssi: f64,

    #[serde(rename = "baro_rate")]
    pub climb_descent_rate: Option<f64>,

    pub squawk: Option<String>,
    pub emergency: Option<String>,

    #[serde(rename = "true_heading")]
    pub true_heading: Option<f64>,

    pub db_flags: Option<i64>,

    #[serde(rename = "mag_heading")]
    pub mag_heading: Option<f64>,

    pub ias: Option<i64>,
    pub tas: Option<i64>,
    pub mach: Option<f64>,
    pub wd: Option<i64>,
    pub ws: Option<i64>,
    pub oat: Option<i64>,
    pub tat: Option<i64>,

    #[serde(rename = "track_rate")]
    pub track_rate: Option<f64>,

    pub roll: Option<f64>,

    #[serde(rename = "nav_altitude_fms")]
    pub nav_altitude_fms: Option<i64>,

    #[serde(rename = "calc_track")]
    pub calc_track: Option<i64>,

    pub gps_ok_before: Option<f64>,
    pub gps_ok_lat: Option<f64>,
    pub gps_ok_lon: Option<f64>,

    #[serde(rename = "rr_lat")]
    pub rr_lat: Option<f64>,

    #[serde(rename = "rr_lon")]
    pub rr_lon: Option<f64>,

    pub last_position: Option<LastPosition>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastPosition {
    pub lat: f64,
    pub lon: f64,
    pub nic: i64,
    pub rc: i64,

    #[serde(rename = "seen_pos")]
    pub seen_pos: f64,
}
