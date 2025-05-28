// use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
// use regex::Regex;
// use reqwest::Client;
// use serde::Deserialize;
// use serde_json::json;
// use std::error::Error;
// use std::sync::{Arc, Mutex};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::TcpStream;
// use warp::Filter;
// use whisper_rs::Whisper;

// fn extract_bible_verse(transcript: &str) -> Option<String> {
//     let re = regex::Regex::new(r"([A-Za-z]+)\s+(\d+):(\d+)").unwrap();
//     if let Some(cap) = re.captures(transcript) {
//         let book = &cap[1];
//         let chapter = &cap[2];
//         let verse = &cap[3];
//         return Some(format!("{} {}:{}", book, chapter, verse));
//     }
//     None
// }

// async fn send_verse_to_server(verse: &str) -> Result<(), Box<dyn Error>> {
//     let client = Client::new();
//     let response = client
//         .post("http://localhost:6500/verse") // Replace with server's address & port
//         .json(&json!({ "bible_verse": verse }))
//         .send()
//         .await?;

//     println!("Sent verse to Computer 2, response: {:?}", response);
//     Ok(())
// }

// async fn send_to_propresenter(verse: &str, host: &str, port: u16) -> Result<(), Box<dyn Error>> {
//     // Create a TCP connection to ProPresenter
//     let mut client_socket = TcpStream::connect((host, port)).await?;

//     // Craft the command for searching a Bible verse
//     let command = json!({
//         "action": "bible.verseSearch",
//         "parameters": {
//             "verse": verse
//         }
//     });

//     // Send the command
//     let command_str = serde_json::to_string(&command)?;
//     client_socket.write_all(command_str.as_bytes()).await?;

//     // Receive the response
//     let mut buffer = [0; 4096];
//     let n = client_socket.read(&mut buffer).await?;
//     let response = String::from_utf8_lossy(&buffer[..n]);

//     println!("Response: {}", response);

//     Ok(())
// }

// fn main() {
//     let host = cpal::default_host();
//     let input_device = host
//         .default_input_device()
//         .expect("No input device available");
//     let input_format = input_device
//         .default_input_format()
//         .expect("Failed to get input format");

//     let whisper_model = Whisper::new(".model").expect("Failed to load model");
//     let verse_regex = Regex::new(r"(?P<book>[A-Za-z]+) (?P<chapter>\d+):(?P<verse>\d+)").unwrap();

//     let input_data = Arc::new(Mutex::new(Vec::new()));

//     let stream = input_device
//         .build_input_stream(
//             &input_format,
//             move |data: &[f32], _| {
//                 let mut input_data = input_data.lock().unwrap();
//                 input_data.extend_from_slice(data);
//             },
//             |err| {
//                 eprintln!("Stream error: {:?}", err);
//             },
//         )
//         .expect("Failed to build stream");

//     stream.play().expect("Failed to play stream");

//     loop {
//         // Here you would normally convert audio data to text
//         let audio_chunk = { input_data.lock().unwrap().clone() };

//         // Mock recognition for demonstration
//         let recognized_text = whisper_model
//             .recognize(&audio_chunk)
//             .expect("Recognition failed");

//         for line in recognized_text.lines() {
//             if let Some(captures) = verse_regex.captures(line) {
//                 let book = &captures["book"];
//                 let chapter = &captures["chapter"];
//                 let verse = &captures["verse"];
//                 println!("Identified scripture: {} {}:{}", book, chapter, verse);
//             }
//         }

//         // Sleep or wait for the next chunk, handle stream
//     }
// }

fn main() {}