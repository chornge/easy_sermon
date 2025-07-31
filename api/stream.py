import asyncio
import os
import json
import queue
import sounddevice as sd

from vosk import Model, KaldiRecognizer
from api.reference import extract_bible_reference
from api.propresenter import stage_display

# Global settings
SAMPLE_RATE = 16000
MODEL_PATH = "models/vosk-model-en-us-0.42-gigaspeech"

# Load Vosk model
if not os.path.exists(MODEL_PATH):
    raise FileNotFoundError(f"Model not found at {MODEL_PATH}")
model = Model(MODEL_PATH)
recognizer = KaldiRecognizer(model, SAMPLE_RATE)

audio_queue = queue.Queue()

detected_verses = []


def start_vosk_stream():
    def callback(indata, frames, time, status):
        if status:
            print("", status, flush=True)
        audio_queue.put(bytes(indata))

    with sd.RawInputStream(
        samplerate=SAMPLE_RATE,
        blocksize=4000,  # smaller block size for lower latency (~0.25s) instead of 8000
        dtype="int16",
        channels=1,
        callback=callback,
    ):
        print("Ready...🎙️...")
        while True:
            data = audio_queue.get()
            if recognizer.AcceptWaveform(data):
                j = recognizer.Result()
            else:
                j = recognizer.PartialResult()

            output = json.loads(j)
            text = output.get("partial") or output.get("text", "")
            text = text.strip().lower()
            if not text:
                continue

            # Skip noise
            if text in {"the"}:
                text = text.replace("the", "").strip()
                continue
            if text in "":
                continue

            print("🔍 Transcript:", text)

            # Detect Bible references
            for ref in extract_bible_reference(text):
                if ref not in detected_verses:
                    detected_verses.append(ref)
                    print("✅ Got:", ref)
                    # asyncio.run(stage_display(ref))
                else:
                    detected_verses.remove(ref)
                    detected_verses.append(ref)
                    # asyncio.run(stage_display(ref))
