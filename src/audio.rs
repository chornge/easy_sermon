use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use vosk::{DecodingState, Model, Recognizer};

use crate::scriptures::bible_verse;

fn process_result(json_str: &str, verses: &Arc<Mutex<Vec<String>>>) {
    let v: serde_json::Value = match serde_json::from_str(json_str) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("JSON parse error: {}", e);
            return;
        }
    };

    // Extract recognized text from "text" or "partial" fields
    let text = v
        .get("text")
        .or_else(|| v.get("partial"))
        .and_then(|val| val.as_str())
        .unwrap_or("")
        .trim()
        .to_lowercase();

    if text.is_empty() {
        return;
    }

    // Filter out noise words
    if text == "the" || text.is_empty() {
        return;
    }

    println!("🔍 Transcript: {}", text);

    for verse in bible_verse(&text) {
        let mut locked_verses = verses.lock().unwrap();
        if !locked_verses.contains(&verse) {
            locked_verses.push(verse.clone());
            println!("✅ Got: {}", verse);
        } else {
            // Move to end to refresh
            locked_verses.retain(|v| v != &verse);
            locked_verses.push(verse.clone());
        }
    }
}

#[allow(dead_code)]
pub fn speech_to_text() -> Result<()> {
    const SAMPLE_RATE: f32 = 16000.0;
    const MODEL_PATH: &str = "models/vosk-model-en-us-0.42-gigaspeech";

    let model = Model::new(MODEL_PATH)
        .ok_or_else(|| anyhow::anyhow!("Model not found or failed to load"))?;
    let recognizer = Recognizer::new(&model, SAMPLE_RATE)
        .ok_or_else(|| anyhow::anyhow!("Failed to create Recognizer"))?;
    let recognizer = Arc::new(Mutex::new(recognizer));

    let (tx, rx) = crossbeam_channel::unbounded::<Vec<i16>>();
    let verses = Arc::new(Mutex::new(Vec::<String>::new()));

    {
        let tx = tx.clone();
        thread::spawn(move || {
            let host = cpal::default_host();
            let device = host.default_input_device().expect("No input device found");
            let config = device
                .default_input_config()
                .expect("No default input config");

            let stream = device
                .build_input_stream(
                    &config.clone().into(),
                    move |data: &[i16], _| {
                        tx.send(data.to_vec()).unwrap();
                    },
                    |err| eprintln!("Stream error: {err}"),
                    None,
                )
                .unwrap();

            stream.play().unwrap();

            loop {
                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    println!("🎙️ Ready to listen...");

    for buffer in rx.iter() {
        let mut rec = recognizer.lock().unwrap();
        match rec.accept_waveform(&buffer) {
            Ok(state) => match state {
                DecodingState::Finalized => {
                    let result = rec.result();
                    let json_str = serde_json::to_string(&result).unwrap();
                    process_result(&json_str, &verses);
                }
                DecodingState::Running => {
                    let partial = rec.partial_result();
                    let json_str = serde_json::to_string(&partial).unwrap();
                    process_result(&json_str, &verses);
                }
                DecodingState::Failed => {
                    eprintln!("Decoding failed");
                }
            },
            Err(e) => {
                eprintln!("accept_waveform error: {:?}", e);
            }
        }
    }

    Ok(())
}
