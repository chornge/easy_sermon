![CI/CD](https://github.com/chornge/easy_sermon/actions/workflows/easy_sermon.yml/badge.svg?branch=main)

# Bible Detection Software

An application that listens to a live audio stream, recognizes spoken Bible verses, and identifies the scripture in a structured format.

## Features

- **Audio Stream Listening**: Captures audio from an input device.
- **Speech Recognition**: Utilizes the Whisper model to convert audio to text.
- **Verse Detection**: Matches recognized text against Bible verse patterns using regex.

## Requirements

- Rust (install via [rustup](https://rustup.rs/))
- Audio input device (microphone)
- Whisper model (ensure a compatible model is downloaded)

### Usage

Build project:

```
cargo build --release
```

Run application:

```
cargo run
```

Speak Bible verses into audio device. The application will identify and print recognized verses in the format:

```
Identified scripture: Book Chapter:Verse
```
