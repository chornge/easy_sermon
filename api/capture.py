import asyncio
import os
import json
import queue
import sounddevice as sd

from vosk import Model, KaldiRecognizer
from api.detect import bible_verse
from api.display import broadcast, offline_bible, stage_display

# Global settings
SAMPLE_RATE = 16000
MODEL_PATH = "models/vosk-model-en-us-0.42-gigaspeech"

# Load Vosk model
if not os.path.exists(MODEL_PATH):
    raise FileNotFoundError(f"Model not found at {MODEL_PATH}")
model = Model(MODEL_PATH)
recognizer = KaldiRecognizer(model, SAMPLE_RATE)

audio_queue = queue.Queue()

verses = ["Genesis 1:1"]  # Initial verse
previous_verse = None


def speech_to_text() -> None:
    global previous_verse

    def callback(indata, frames, time, status) -> None:
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

            # Detect Bible verses
            for verse in bible_verse(text):
                if verse != previous_verse:
                    print("✅ Got:", verse)
                    verses.append(verse)
                    previous_verse = verse
                    full_text = offline_bible(verse)
                    asyncio.run(
                        broadcast(json.dumps({"reference": verse, "text": full_text}))
                    )
                    asyncio.run(stage_display(full_text))
