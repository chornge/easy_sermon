![CI/CD](https://github.com/chornge/easy_sermon/actions/workflows/build.yml/badge.svg?branch=main)

An application that listens to a live audio stream, recognizes any Bible verses, and displays the scripture in a structured format.

## Architecture

![Design Doc](design_doc.excalidraw.png)

- **Audio Stream Listening**: Captures audio from an input device.
- **Speech Recognition**: Utilizes the Vosk model to convert audio to text.
- **Verse Detection**: Matches recognized text against Bible verse patterns using Regex.

## Requirements

- Rust (install via [rustup](https://rustup.rs/))
- Python (`v3.10`)
- Audio input device (microphone, etc)
- FFMPEG (`brew install ffmpeg` on macOS)
- [Vosk](https://github.com/alphacep/vosk-api) (`small`, `medium`, `large`, etc) - downloaded as part of build script.
- [Whisper](https://github.com/openai/whisper) (`tiny.en`, `base.en`, `small.en`, etc) - not needed unless testing with audio files/chunking audio streams.

## Usage

```
git clone https://github.com/chornge/easy_sermon.git
cd ~/PATH/to/easysermon
python3 -m venv venv
source venv/bin/activate
pip install -r api/requirements.txt
```

On macOS only - manually install system certs, run:

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
"as it says in john three verse sixteen" -> [John 3:16]
"let's take a look at romans five" -> [Romans 5:1]
"combining exodus one one leviticus one one and job three one into a module" -> [Exodus 1:1, Leviticus 1:1, Job 3:1]
```

Sometimes you may want to see the list of all connected audio devices. To show all input and output devices, run:

```
python3 api/devices.py
```

To manually download and test a Vosk model (streaming), run:

```
python3 api/download_vosk.py
python3 api/reference.py
```

To manually download and test a Whisper model (chunking), run:

```
python3 api/download_whisper.py
python3 api/chunk.py
```
