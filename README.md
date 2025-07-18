![CI/CD](https://github.com/chornge/easy_sermon/actions/workflows/build.yml/badge.svg?branch=main)

An application that listens to a live audio stream, recognizes any Bible verses, and displays the scripture in a structured format.

## Architecture

![Design Doc](design_doc.excalidraw.png)

- **Audio Stream Listening**: Captures audio from an input device.
- **Speech Recognition**: Utilizes the Vosk model to convert audio to text.
- **Verse Detection**: Matches texts against Bible verses using fuzzy-matching & regular expressions.

## Requirements

- Rust (install via [rustup](https://rustup.rs/))
- Python (`v3.10`)
- Audio input device (microphone, etc)
- FFMPEG (`brew install ffmpeg` on macOS)
- [Vosk](https://github.com/alphacep/vosk-api) (`small`, `medium`, `large`, etc) - downloaded as part of build script.

## Usage

```
git clone https://github.com/chornge/easy_sermon.git
cd ~/PATH/to/easy_sermon
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
"inside third john one verse two" -> [3 John 1:2]
"let's take a look at romans five" -> [Romans 5:1]
"as it says in john three verse sixteen" -> [John 3:16]
"at genesis chapter two verse eight" -> [Genesis 2:8]
"first corinthians thirteen verse four" -> [1 Corinthians 13:4]
"viewing psalm one hundred and five verse forty one" -> [Psalm 105:41]
```

Sometimes you may want to see the list of all connected audio devices. To show all input and output devices, run:

```
python3 api/devices.py
```

To manually download and test a Vosk model (vosk-model, vosk-model-small, etc), run:

```
python3 api/download_vosk.py
python3 api/reference.py
```
