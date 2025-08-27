// mod capture;
mod detect;
mod display;

use anyhow::Result;
use dotenv::dotenv;
use std::env;
use std::process::Command;

// use crate::capture::speech_to_text;
// use crate::detect::bible_verse;
// use crate::display::stage_display;

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

    // for &line in &[
    //     "for the hope we have in john three verse sixteen",
    //     "keeping in mind the consequences in romans six verse twenty three",
    //     "nothing compares to the grace in ephesians two verse eight",
    //     "showing how near salvation is in romans ten verse nine",
    //     "finding true life in john fourteen verse six",
    //     "and our identity in galatians two verse twenty",
    //     "we are never too far gone in first john one verse nine",
    //     "for we celebrate a fresh start in second corinthians five verse seventeen",
    //     "finding the blueprint for peace in philippians four verses six and seven",
    //     "while on the great commission in matthew twenty eight verse nineteen through twenty",
    // ] {
    //     let reference = bible_verse(line);
    //     println!("\n🔍 Audio: {line} \n✅ Got: {reference:?}");

    //     // Send verse(s) to Stage Display
    //     for verse in &reference {
    //         let _ = stage_display(verse).await;
    //     }
    // }

    Ok(())
}
