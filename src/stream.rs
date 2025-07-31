use rodio::{Decoder, OutputStream, Sink, Source};
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Instant;
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::{get_codecs, get_probe};
use vosk::CompleteResult;
use vosk::Model;
use vosk::Recognizer;

/* External crates needed in Cargo.toml:
crossbeam-channel = "0.5.15"
rodio = "0.17.3"
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.141"
symphonia = { version = "0.5.2", features = ["mp3", "wav", "flac"] }
vosk = "0.3.1"
*/

pub fn run_transcription_loop(audio_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let model = Model::new("model");
    let mut recognizer = Recognizer::new(&model.unwrap(), 16000.0);

    let file = File::open(audio_path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let hint = Hint::new();
    let probed = get_probe().format(
        &hint,
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    )?;

    let mut format = probed.format;
    let track = format
        .default_track()
        .ok_or_else(|| SymphoniaError::DecodeError("No default track"))?;

    let decoder = get_codecs().make(&track.codec_params, &DecoderOptions::default())?;

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle);

    let file = File::open(audio_path)?;
    let source = Decoder::new(BufReader::new(file))?.buffered();
    sink?.append(source);

    let (tx, rx) = channel();
    thread::spawn(move || loop {
        match format.next_packet() {
            Ok(packet) => match decoder.decode(&packet) {
                Ok(audio_buf) => {
                    if let AudioBufferRef::F32(buf) = audio_buf {
                        let samples: Vec<i16> = buf
                            .chan(0)
                            .iter()
                            .map(|s| (*s * i16::MAX as f32) as i16)
                            .collect();
                        let _ = tx.send(samples);
                    }
                }
                Err(_) => continue,
            },
            Err(_) => break,
        }
    });

    let mut recent: VecDeque<String> = VecDeque::with_capacity(5);
    let mut last_seen = std::collections::HashMap::new();
    let mut last_detection_time = Instant::now();

    while let Ok(samples) = rx.recv() {
        recognizer.expect("REASON").accept_waveform(&samples);

        let partial = recognizer.expect("REASON").partial_result();
        let t = partial.partial.trim().to_lowercase();

        if !t.is_empty() && t.len() > 1 {
            if let Some(pos) = recent.iter().position(|x| *x == t) {
                if pos == recent.len() - 1 {
                    // already last, do nothing
                } else {
                    recent.remove(pos);
                    recent.push_back(t.clone());
                    println!("RESHUFFLED TO BACK: {}", t);
                }
            } else {
                recent.push_back(t.clone());
                if recent.len() > 5 {
                    recent.pop_front();
                }
                println!("NEW: {}", t);
            }
        }

        if recognizer.final_result_ready() {
            match recognizer.expect("REASON").result() {
                CompleteResult::Single(text) => {
                    let final_text = text.text.trim().to_lowercase();
                    if !final_text.is_empty() {
                        println!("FINAL: {}", final_text);
                    }
                }
                CompleteResult::Multiple(alternatives) => {
                    if let Some(first) = alternatives.alternatives.first() {
                        let final_text = first.text.trim().to_lowercase();
                        if !final_text.is_empty() {
                            println!("FINAL: {}", final_text);
                        }
                    }
                }
            }
        }
    }

    sink?.sleep_until_end();
    Ok(())
}
