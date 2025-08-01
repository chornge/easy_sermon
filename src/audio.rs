use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, StreamConfig};
use std::io::{self, Write};

// use crate::reference::extract_bible_reference;
// use std::error::Error;
// use std::sync::{Arc, Mutex};
// use vosk::{DecodingState, Model, Recognizer};

fn fetch_devices() {
    let host = cpal::default_host();
    let devices = match host.devices() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error getting devices: {e}");
            return;
        }
    };

    for (i, device) in devices.enumerate() {
        let name = device.name().unwrap_or_else(|_| "Unknown".to_string());

        let supported_configs = device.supported_input_configs();
        let is_input = supported_configs.is_ok_and(|mut sc| sc.next().is_some());

        println!(
            "{}: {} ({})",
            i,
            name,
            if is_input { "Input ✅" } else { "Output 🛑" }
        );
    }
}

#[allow(dead_code)]
pub fn speech_to_text() -> Result<(), Box<dyn std::error::Error>> {
    fetch_devices();

    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No input devices available");

    println!("Using: {}", device.name()?);

    let cfg = device.default_input_config()?;
    let cfg: StreamConfig = cfg.into();
    println!("Default input config: {cfg:?}");

    let stream = match device.default_input_config()?.sample_format() {
        SampleFormat::F32 => build_stream_f32(&device, &cfg)?,
        SampleFormat::I16 => build_stream_i16(&device, &cfg)?,
        SampleFormat::U16 => build_stream_u16(&device, &cfg)?,
        _ => return Err("Unsupported sample format".into()),
    };

    stream.play()?;

    // Block until 'Enter' is pressed
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    // Extract Bible reference
    // let text = "as we turn to john chapter three verse sixteen we see the love God has for us";
    // let reference = extract_bible_reference(text);
    // println!("Input: {text:?}, Verse: {reference:?}");

    Ok(())
}

fn build_stream_f32(
    device: &cpal::Device,
    cfg: &StreamConfig,
) -> Result<cpal::Stream, Box<dyn std::error::Error>> {
    let channels = cfg.channels as usize;
    let err_fn = |e| eprintln!("Stream error: {e}");

    let stream = device.build_input_stream(
        cfg,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let avg: f32 = data
                .chunks(channels)
                .map(|frame| frame[0].abs())
                .sum::<f32>()
                / (data.len() / channels).max(1) as f32;
            print!("\rMic Level: {avg:.4}");
            io::stdout().flush().unwrap();
        },
        err_fn,
        None,
    )?;

    Ok(stream)
}

fn build_stream_i16(
    device: &cpal::Device,
    cfg: &StreamConfig,
) -> Result<cpal::Stream, Box<dyn std::error::Error>> {
    let channels = cfg.channels as usize;
    let err_fn = |e| eprintln!("Stream error: {e}");

    let stream = device.build_input_stream(
        cfg,
        move |data: &[i16], _: &cpal::InputCallbackInfo| {
            let avg: f32 = data
                .chunks(channels)
                .map(|frame| (frame[0] as f32 / i16::MAX as f32).abs())
                .sum::<f32>()
                / (data.len() / channels).max(1) as f32;
            print!("\rMic Level: {avg:.4}");
            io::stdout().flush().unwrap();
        },
        err_fn,
        None,
    )?;

    Ok(stream)
}

fn build_stream_u16(
    device: &cpal::Device,
    cfg: &StreamConfig,
) -> Result<cpal::Stream, Box<dyn std::error::Error>> {
    let channels = cfg.channels as usize;
    let err_fn = |e| eprintln!("Stream error: {e}");

    let stream = device.build_input_stream(
        cfg,
        move |data: &[u16], _: &cpal::InputCallbackInfo| {
            let avg: f32 = data
                .chunks(channels)
                .map(|frame| {
                    let s = frame[0] as f32 / u16::MAX as f32;
                    (s * 2.0 - 1.0).abs()
                })
                .sum::<f32>()
                / (data.len() / channels).max(1) as f32;
            print!("\rMic Level: {avg:.4}");
            io::stdout().flush().unwrap();
        },
        err_fn,
        None,
    )?;

    Ok(stream)
}
