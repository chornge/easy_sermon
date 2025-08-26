use futures_util::StreamExt;
use once_cell::sync::Lazy;
use reqwest::Client;
use std::{
    error::Error,
    fs::{self, File},
    io::{Cursor, Write, copy},
    path::Path,
};
use zip::ZipArchive;

/* Models (ranked by accuracy) - Replace with a different model if needed
"https://alphacephei.com/vosk/models/vosk-model-en-us-0.42-gigaspeech.zip"  # 4.1 GB | 94.36% accurate
"https://alphacephei.com/vosk/models/vosk-model-en-us-0.22.zip"             # 2.9 GB | 94.31% accurate
"https://alphacephei.com/vosk/models/vosk-model-en-us-0.22-lgraph.zip"      # 215 MB | 92.18% accurate
"https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip"       # 71 MB  | 90.15% accurate
*/

const MODEL_DIR: &str = "models";
const MODEL_PATH: &str = "vosk-model-en-us-0.42-gigaspeech";
static MODEL_FULL_PATH: Lazy<String> = Lazy::new(|| format!("{MODEL_DIR}/{MODEL_PATH}"));

#[tokio::main]
async fn main() {
    println!(
        "cargo:rustc-env=VOSK_MODEL_PATH={}",
        MODEL_FULL_PATH.as_str()
    );

    println!("cargo:rustc-link-search=native=./native");
    // println!("cargo:rustc-link-lib=dylib=vosk");
    println!("cargo:rerun-if-changed=src/build.rs");

    if std::env::var("CI").is_ok() {
        println!("CI/CD pipeline detected, skipping model download.");
        return;
    }

    if Path::new(MODEL_FULL_PATH.as_str()).exists() {
        println!("Model already exists at {}", MODEL_FULL_PATH.as_str());
        return;
    }

    let model_url = format!(
        "https://alphacephei.com/vosk/{}.zip",
        MODEL_FULL_PATH.as_str()
    );
    println!("Downloading model from: {model_url}");

    let bytes = match download_zip(&model_url).await {
        Ok(b) => b,
        Err(e) => {
            panic!("Failed to download model: {e}");
        }
    };

    if let Err(e) = extract_zip(bytes, MODEL_DIR) {
        panic!("Failed to extract model: {e}");
    }

    // Create .env if it doesn't exist
    let env_path = Path::new(".env");
    if !env_path.exists() {
        let mut file = File::create(env_path).expect("Failed to create .env file");
        writeln!(file, "VOSK_MODEL_PATH={}", MODEL_FULL_PATH.as_str())
            .expect("Failed to write to .env file");
    } else {
        // If .env exists, ensure it has the correct path
        let mut file = File::options()
            .append(true)
            .open(env_path)
            .expect("Failed to open .env file");
        writeln!(file, "VOSK_MODEL_PATH={}", MODEL_FULL_PATH.as_str())
            .expect("Failed to write to .env file");
    }

    println!("Model download and Path declaration complete.");
}

async fn download_zip(url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?.error_for_status()?;
    let mut data = Vec::new();
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        data.extend_from_slice(&chunk?);
    }

    Ok(data)
}

fn extract_zip(bytes: Vec<u8>, output_dir: &str) -> Result<(), Box<dyn Error>> {
    let reader = Cursor::new(bytes);
    let mut zip = ZipArchive::new(reader)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => Path::new(output_dir).join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&outpath)?;
            copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}
