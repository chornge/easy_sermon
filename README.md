![CI/CD](https://github.com/chornge/easy_sermon/actions/workflows/build.yml/badge.svg?branch=main)

An application that listens to a live audio stream, recognizes any Bible verses, and displays the Scripture.

### Demo

https://github.com/user-attachments/assets/c03a0ed5-18c1-4bc8-a1c5-6fad6f200304

## Architecture

![Design Doc](design_doc.excalidraw.png)

- **Audio Stream**: Capture audio from an input device.
- **Speech Recognition**: Utilize the Vosk speech model to convert audio to text.
- **Verse Detection**: Match texts against Bible verses using regular expressions (regex).
- **Display Output**: Send detected verses to Stage Display for live presentation.

## Requirements

- Python (v3.10)
- Audio input device (microphone, etc)
- Enough space for [Vosk](https://alphacephei.com/vosk/models) - downloaded during build (~4GB)

### Windows

- [Git for Windows](https://git-scm.com/downloads/win) Portable edition (includes Git & OpenSSL)
- Rust ([rustup](https://rustup.rs/))

### Mac

- Command Line Tools - `sudo xcode select --install`
- [brew](https://brew.sh) (optional for installing OpenSSL)
- OpenSSL - `brew install openssl`
- Rust ([rustup](https://rustup.rs/))

## Usage

```
git clone https://github.com/chornge/easy_sermon.git
cd easy_sermon
python -m venv venv
```

Activate virtual environment:

`source venv/bin/activate` (macOS)

`source venv/Scripts/activate` (windows)

```
pip install -r api/requirements.txt
```

macOS only - if missing, install system certs:

```
chmod +x get-certificates.sh

bash get-certificates.sh
```

To receive verses on Stage Display, enable Pro-Presenter API (TCP with port 54346)

ProPresenter > Settings > Network

<img width="321" height="400" alt="propresenter_tcp" src="https://github.com/user-attachments/assets/b3cc630a-0e0a-4826-b3fc-b5625ed68506" />

```
cargo run --release
```

Wait until application shows `Ready...🎙️...`

Navigate to `http://localhost` in the browser. Allow microphone & clipboard access (when prompted)

Start speaking:

```
"for the hope we have in john three verse sixteen" -> ['John 3:16']
"keeping in mind the consequences in romans six verse twenty three" -> ['Romans 6:23']
"nothing compares to the grace in ephesians two verse eight" -> ['Ephesians 2:8']
"showing how near salvation is in romans ten verse nine" -> ['Romans 10:9']
"finding true life in john fourteen verse six" -> ['John 14:6']
"and our identity in galatians two verse twenty" -> ['Galatians 2:20']
"we are never too far gone in first john one verse nine" -> ['1 John 1:9']
"for we celebrate a fresh start in second corinthians five verse seventeen" -> ['2 Corinthians 5:17']
"finding the blueprint for peace in philippians four verses six and seven" -> ['Philippians 4:6-7']
"while on the great commission in matthew twenty eight verse nineteen through twenty" -> ['Matthew 28:19-20']
```

## Limitations

• The word `"verse"` must be present for accurate Bible verse extraction.

• Untested on Windows and Linux. Also untested on ARM64 (M1/M2/M3, etc).
