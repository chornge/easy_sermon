import os
from pathlib import Path
import queue
import sounddevice as sd

from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.templating import Jinja2Templates
from vosk import Model, KaldiRecognizer

from api.reference import extract_bible_reference

# Global settings
SAMPLE_RATE = 16000
MODEL_PATH = "models/vosk-model-en-us-0.22"
BASE_DIR = Path(__file__).resolve().parent.parent
templates = Jinja2Templates(directory=os.path.join(BASE_DIR, "templates"))

app = FastAPI()

# Load Vosk model
if not os.path.exists(MODEL_PATH):
    raise FileNotFoundError(f"Model not found at {MODEL_PATH}")
model = Model(MODEL_PATH)
recognizer = KaldiRecognizer(model, SAMPLE_RATE)

# Shared queue and result buffer
audio_queue = queue.Queue()
result_text = ""

detected_verses = []


def start_vosk_stream():
    def callback(indata, frames, time, status):
        if status:
            print(status, flush=True)
        audio_queue.put(bytes(indata))

    with sd.RawInputStream(
        samplerate=SAMPLE_RATE,
        blocksize=8000,
        dtype="int16",
        channels=1,
        callback=callback,
    ):
        print("Ready...🎙️...")
        while True:
            data = audio_queue.get()
            if recognizer.AcceptWaveform(data):
                result = recognizer.Result()
                text = eval(result).get("text", "")
                print("🔍 Transcribed text:", text)
                if text:
                    references = extract_bible_reference(text)
                    print("✅ Got:", references)
                    for ref in references:
                        if ref not in detected_verses:
                            detected_verses.append(ref)


@app.get("/transcript")
def get_transcript():
    return {"transcript": result_text.strip()}


@app.get("/", response_class=HTMLResponse)
async def home(request: Request):
    return templates.TemplateResponse(
        "index.html", {"request": request, "verses": detected_verses}
    )
