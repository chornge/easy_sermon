import os
import json
import queue

# import asyncio
from pathlib import Path
import sounddevice as sd

from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.templating import Jinja2Templates
from vosk import Model, KaldiRecognizer

from api.reference import extract_bible_reference
from api.propresent import send_text_to_propresenter

# Global settings
SAMPLE_RATE = 16000
MODEL_PATH = "models/vosk-model-en-us-0.42-gigaspeech"
BASE_DIR = Path(__file__).resolve().parent.parent
templates = Jinja2Templates(directory=os.path.join(BASE_DIR, "templates"))

app = FastAPI()

# Load Vosk model
if not os.path.exists(MODEL_PATH):
    raise FileNotFoundError(f"Model not found at {MODEL_PATH}")
model = Model(MODEL_PATH)
recognizer = KaldiRecognizer(model, SAMPLE_RATE)

audio_queue = queue.Queue()
result_text = ""

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

            print("🔍 Transcribed text:", text)
            for ref in extract_bible_reference(text):
                if ref not in detected_verses:
                    detected_verses.append(ref)
                    print("✅ Got:", ref)
                    # asyncio.run(send_text_to_propresenter(ref))


@app.get("/transcript")
def get_transcript():
    return {"transcript": detected_verses}


@app.get("/", response_class=HTMLResponse)
async def home(request: Request):
    return templates.TemplateResponse(
        "index.html", {"request": request, "verses": detected_verses}
    )
