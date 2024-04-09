use reqwest::get;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write, Error};
use flate2::read::GzDecoder;
use std::path::Path;

pub async fn read_api() -> Result<(), Error> {
    // Define base URL
    let base_url = "https://samples.adsbexchange.com/readsb-hist/2024/01/01/";

    let start_num = 0;
    let increment = 5;
    let num_files = 50;

    for i in 0..num_files {
        let num = format!("{:06}", start_num + i * increment);
        let url = format!("{}{}Z.json.gz", base_url, num);
        let filename = format!("{}Z.json.gz", num);

        let response = match get(&url).await {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Failed to perform HTTP request: {}", e);
                continue; // Continue to the next iteration
            }
        };

        if response.status().is_success() {

            let mut file = match File::create(format!("{}.json.gz", num)) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Failed to create file {}: {}", num, e);
                    continue; // Continue to the next iteration
                }
            };

            if let Err(e) = file.write_all(&response.bytes().await.unwrap()) {
                eprintln!("Failed to write to file {}: {}", num, e);
                continue; // Continue to the next iteration
            }
            println!("File {} downloaded successfully", num);
        } else {
            println!("Failed to download file {}: {}", num, response.status());
        }
    }
    let _ = show_files().await;
    Ok(())
}

async fn show_files() {
    let dir_path = "C:\\Users\\Edward\\RustroverProjects\\parse_adsb";
    let extension = "gz";

    if let Ok(entries) = fs::read_dir(dir_path) {
        // Iterate over directory entries
        for entry in entries {
            if let Ok(entry) = entry {
                // Get the path of the entry
                let path = entry.path();

                if path.is_file() && path.extension().map_or(false, |ext| ext == extension) {
                    let _ = unpack(&path.to_str().unwrap());
                }
            } else {
                eprintln!("Failed to get directory entry");
            }
        }
    } else {
        eprintln!("Failed to read directory contents");
    }
}
fn unpack(filename: &str) -> io::Result<()>  {

    let file = File::open(&filename)?;

    let mut decoder = GzDecoder::new(file);
    let mut decompressed_data = Vec::new();

    decoder.read_to_end(&mut decompressed_data)?;

    let mut file_out = File::create(&filename[0..&filename.len() - 3])?;
    file_out.write_all(&decompressed_data)?;

    println!("File decompressed successfully.");
    let _ = fs::remove_file(filename);

    Ok(())
}