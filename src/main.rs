mod adsb_data;
mod api_reader;
mod database;

use crate::adsb_data::Root;
use serde_json;
use std::fs;

#[tokio::main]
async fn main() {
    let _ = api_reader::read_api().await;
    let path = "C:\\Users\\Edward\\Downloads\\000015Z.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let adsb: Root = serde_json::from_str(&data).expect("Trouble");
    database::insert_adsb(&adsb).await;
}
