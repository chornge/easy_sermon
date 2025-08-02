mod audio;
mod propresenter;
mod scriptures;

use anyhow::Result;
use dotenv::dotenv;
use std::{env, process::Command};

// use crate::audio::speech_to_text;
// use crate::propresenter::stage_display;
// use crate::scriptures::bible_verse;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env if present
    dotenv().ok();
    env::var("VOSK_MODEL_PATH")
        .expect("Set VOSK_MODEL_PATH environment variable to the Vosk model directory path");

    // Start audio stream
    // let audio_stream = speech_to_text();
    // if audio_stream.is_err() {
    //     panic!(
    //         "Failed to start audio stream: {}",
    //         audio_stream.unwrap_err()
    //     );
    // }

    // for &line in &[
    //     "at genesis chapter two verses eight and nine",
    //     "as it says in john three verse sixteen",
    //     "the book of ezekiel chapter thirty three verse two",
    //     "going back to psalm one hundred five verse forty",
    //     "first corinthians thirteen verse four",
    //     "again in third john one verse two",
    //     "open your bibles to revelations twenty two verse three",
    // ] {
    //     let reference = bible_verse(line);
    //     println!("🔍 Audio: {line} \n✅ Got: {reference:?} \n");
    // }

    // Start API server
    let api_server = Command::new("uvicorn")
        .arg("api.main:app")
        .arg("--host")
        .arg("0.0.0.0")
        .arg("--port")
        .arg("80")
        .arg("--log-level")
        .arg("warning")
        .status()
        .expect("Failed to run api/main.py");

    if !api_server.success() {
        panic!("API server failed to start");
    }

    // Send message to stage display
    // let _ = stage_display("Genesis 1:2-9").await;

    Ok(())
}
