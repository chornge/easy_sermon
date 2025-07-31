use crate::devices::list_input_outputs;
use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, StreamConfig};
use std::io::{self, Write};

// use std::error::Error;
// use std::sync::{Arc, Mutex};
// use vosk::{DecodingState, Model, Recognizer};

#[allow(dead_code)]
pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    // List input/output devices
    list_input_outputs();

    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No input device available");

    println!("Using input device: {}", device.name()?);

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

    // Block until Enter pressed
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
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
