use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::{unbounded, Receiver};
use reference::extract_bible_reference;
use serde::Serialize;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};
use vosk::{KaldiRecognizer, Model};

/* External crates needed in Cargo.toml:
actix-web = "4"
crossbeam-channel = "0.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
vosk = "0.3"
*/

const SAMPLE_RATE: f32 = 16_000.0;
const BLOCK_SIZE: usize = 4000;
const MODEL_PATH: &str = "models/vosk-model-en-us-0.42-gigaspeech";

#[derive(Clone)]
struct AppState {
    detected: Arc<Mutex<Vec<String>>>,
}

#[get("/transcript")]
async fn get_transcript(state: web::Data<AppState>) -> impl Responder {
    let verses = state.detected.lock().unwrap().clone();
    HttpResponse::Ok().json(Transcript { transcript: verses })
}

#[get("/")]
async fn home(state: web::Data<AppState>) -> impl Responder {
    let verses = state.detected.lock().unwrap().clone();
    let list = verses
        .iter()
        .map(|v| format!("<li>{}</li>", v))
        .collect::<String>();
    let body = format!(
        "<html><head><title>Detected Verses</title></head><body><h1>Detected Verses</h1><ul>{}</ul></body></html>",
        list
    );
    HttpResponse::Ok().content_type("text/html").body(body)
}

/// Thread to receive audio buffers and run Vosk recognition + Extraction
fn start_vosk_stream(rx: Receiver<Vec<i16>>, detected: Arc<Mutex<Vec<String>>>) {
    // Initialize Vosk
    if !PathBuf::from(MODEL_PATH).exists() {
        panic!("Model not found at {}", MODEL_PATH);
    }
    let model = Model::new(MODEL_PATH).expect("Failed to load Vosk model");
    let mut recognizer = KaldiRecognizer::new(&model, SAMPLE_RATE).unwrap();

    loop {
        if let Ok(buffer) = rx.recv() {
            // Convert i16 slice to raw bytes
            let bytes: &[u8] = unsafe {
                std::slice::from_raw_parts(
                    buffer.as_ptr() as *const u8,
                    buffer.len() * std::mem::size_of::<i16>(),
                )
            };
            let json = if recognizer.accept_waveform(bytes) {
                recognizer.result()
            } else {
                recognizer.partial_result()
            };

            // Parse Vosk JSON output
            if let Ok(output) = serde_json::from_str::<serde_json::Value>(&json) {
                let text = output
                    .get("text")
                    .or_else(|| output.get("partial"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim()
                    .to_lowercase();
                if !text.is_empty() && text != "the" {
                    println!("🔍 Transcribed text: {}", text);

                    // Extract references and add new ones
                    for reference in extract_bible_reference(&text) {
                        let mut guard = detected.lock().unwrap();
                        if !guard.contains(&reference) {
                            guard.push(reference.clone());
                            println!("✅ Got: {}", reference);
                        }
                    }
                }
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Shared state
    let detected = Arc::new(Mutex::new(Vec::new()));
    let state = AppState {
        detected: detected.clone(),
    };

    // Channel for audio
    let (tx, rx) = unbounded::<Vec<i16>>();

    // Spawn Vosk thread
    let vosk_detected = detected.clone();
    thread::spawn(move || {
        start_vosk_stream(rx, vosk_detected);
    });

    // Setup audio input with CPAL
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No input device available");
    let config = device
        .default_input_config()
        .expect("Failed to get default input config");
    let sample_format = config.sample_format();
    let stream_config = cpal::StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(SAMPLE_RATE as u32),
        buffer_size: cpal::BufferSize::Fixed(BLOCK_SIZE as u32),
    };

    // Build and play input stream
    let tx_clone = tx.clone();
    let err_fn = |err| eprintln!("Stream error: {}", err);
    let stream = match sample_format {
        cpal::SampleFormat::I16 => device.build_input_stream(
            &stream_config,
            move |data: &[i16], _| {
                let _ = tx_clone.send(data.to_vec());
            },
            err_fn,
        ),
        cpal::SampleFormat::U16 => device.build_input_stream(
            &stream_config,
            move |data: &[u16], _| {
                let converted: Vec<i16> = data.iter().map(|&u| (u as i16)).collect();
                let _ = tx_clone.send(converted);
            },
            err_fn,
        ),
        cpal::SampleFormat::F32 => device.build_input_stream(
            &stream_config,
            move |data: &[f32], _| {
                let converted: Vec<i16> =
                    data.iter().map(|&f| (f * i16::MAX as f32) as i16).collect();
                let _ = tx_clone.send(converted);
            },
            err_fn,
        ),
    }?;

    stream.play()?;

    println!("Ready...🎙️...");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(get_transcript)
            .service(home)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
