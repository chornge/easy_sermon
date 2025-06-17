![CI/CD](https://github.com/chornge/easy_sermon/actions/workflows/build.yml/badge.svg?branch=main)

An application that listens to a live audio stream, recognizes any Bible verses, and displays the scripture in a structured format.

## Features

- **Audio Stream Listening**: Captures audio from an input device.
- **Speech Recognition**: Utilizes the Whisper model to convert audio to text (listens in 5 second chunks).
- **Verse Detection**: Matches recognized text against Bible verse patterns using regex.

## Requirements

- Rust (install via [rustup](https://rustup.rs/))
- Python (`v3.10`)
- Audio input device (microphone, etc)
- FFMPEG (`brew install ffmpeg` on macOS)
- [Whisper](https://github.com/openai/whisper) (`tiny.en`, `base.en`, `small.en`, etc)

## Usage

```
git clone https://github.com/chornge/easy_sermon.git
```

On macOS, sometimes system certs need to be manually installed:

```
chmod +x get-certificates.sh

bash get-certificates.sh
```

Run:

```
cargo run --release
```

Navigate to `http://localhost` on the browser. Allow microphone access (if the prompt pops up).

```
"first john two and three" -> [1 John 2:3]
"second peter one verse two" -> [2 Peter 1:2]
"genesis one one exodus two two job three three" -> [Genesis 1:1, Exodus 2:2, Job 3:3]
```

Sometimes you may want to see the list of all connected audio devices. To show all input and output devices, run:

```
python3 api/devices.py
```
