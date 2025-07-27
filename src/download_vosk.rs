use std::fs::File;
use std::io::{Cursor, Read};
use zip::ZipArchive;

/* External crates needed in Cargo.toml:
reqwest = { version = "0.11", features = ["blocking"] }
zip = "0.6"
tempfile = "3.10"
*/

/* Models (ranked by accuracy):
"https://alphacephei.com/vosk/models/vosk-model-en-us-0.22.zip"           # 2.9 GB | 94.31% accurate
"https://alphacephei.com/vosk/models/vosk-model-en-us-0.22-lgraph.zip"    # 215 MB | 92.18% accurate
"https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip"     # 71 MB  | 90.15% accurate
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Change if you want a smaller model
    let url = "https://alphacephei.com/vosk/models/vosk-model-en-us-0.42-gigaspeech.zip"; // 4.1 GB | 94.36% accurate

    // Download ZIP file
    println!("Downloading model from: {}", url);
    let response = reqwest::blocking::get(url)?;
    let bytes = response.bytes()?;

    // Read ZIP content from memory
    let reader = Cursor::new(bytes);
    let mut zip = ZipArchive::new(reader)?;

    // Extract files to "models/" directory
    println!("Extracting...");
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => std::path::Path::new("models").join(path),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    println!("Done.");
    Ok(())
}
