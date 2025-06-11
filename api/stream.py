import whisper
import sounddevice as sd
import numpy as np
import tempfile
import scipy.io.wavfile
import time
import warnings

from reference import extract_bible_references

model = whisper.load_model("base.en")

# Config Settings
duration = 5  # seconds per chunk
sample_rate = 16000
warnings.filterwarnings("ignore", message="FP16 is not supported on CPU*")


def record_audio(duration, sample_rate):
    print(f"Recording {duration}s...")
    audio = sd.rec(
        int(duration * sample_rate), samplerate=sample_rate, channels=1, dtype="float32"
    )
    sd.wait()
    return audio.squeeze()


def transcribe_audio(audio):
    with tempfile.NamedTemporaryFile(suffix=".wav") as f:
        scipy.io.wavfile.write(f.name, sample_rate, audio)
        result = model.transcribe(f.name)
    return result["text"]


print("Listening... Press Ctrl+C to stop.")
try:
    while True:
        audio_chunk = record_audio(duration, sample_rate)
        text = transcribe_audio(audio_chunk)
        print(">>", extract_bible_references(text.strip()))
except KeyboardInterrupt:
    print("\nStopped.")
