mod adsb_data;
mod api_reader;
mod database;

use crate::adsb_data::Root;
use serde_json;
use std::fs;

#[tokio::main]
async fn main() {
    //let _ = api_reader::read_api().await;

    let paths = fs::read_dir("C:\\Users\\Edward\\RustroverProjects\\parse_adsb").unwrap();

    for the_path in paths {
        let path = the_path.unwrap().path();

        match path.extension() {
            Some(p) if p == "json" => {
                let full_path = &path.into_os_string().into_string().unwrap();
                let data = fs::read_to_string(full_path).expect("Unable to read file");
                let adsb: Root = serde_json::from_str(&data).expect("Trouble");
                database::insert_adsb(&adsb).await;
            },
            _ => (),
        }
    }
}

// fn folder_search() {
//     let paths = fs::read_dir("C:\\Users\\Edward\\RustroverProjects\\parse_adsb").unwrap();
//
//     for path in paths {
//         let path = path.unwrap().path();
//
//         match path.extension() {
//             Some(p) if p == "json" => println!("{:?}", path.file_name().unwrap()),
//             _ => (),
//         }
//     }
// }
