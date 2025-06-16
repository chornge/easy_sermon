![CI/CD](https://github.com/chornge/easy_sermon/actions/workflows/build.yml/badge.svg?branch=main)

An application that listens to a live audio stream, recognizes any Bible verses, and identifies the scripture in a structured format.

## Features

- **Audio Stream Listening**: Captures audio from an input device.
- **Speech Recognition**: Utilizes the Whisper model to convert audio to text.
- **Verse Detection**: Matches recognized text against Bible verse patterns using regex.

## Requirements

- Rust (install via [rustup](https://rustup.rs/))
- Audio input device (microphone)
- Whisper model (`base.en`, `small.en`, `tiny.en`, etc)

### Usage

Build:

```
cargo build --release
```

Run:

```
cargo run --release
```

Speak Bible verses into the audio device. The application will identify and print the recognized verses:

```
"first john two and three" -> 1 John 2:3
"second peter one verse two" -> 2 Peter 1:2
"genesis one one exodus two two job three three" -> [Genesis 1:1, Exodus 2:2, Job 3:3]
```
