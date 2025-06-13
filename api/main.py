from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.templating import Jinja2Templates

import os
from pathlib import Path
import threading
from contextlib import asynccontextmanager

from api.stream import stream_bible_verses

BASE_DIR = Path(__file__).resolve().parent.parent
templates = Jinja2Templates(directory=os.path.join(BASE_DIR, "templates"))

detected_verses = []


@asynccontextmanager
async def lifespan(app: FastAPI):
    def run_stream():
        print("🎤 Audio stream starting")
        for verse_list in stream_bible_verses():
            print("✅ Got:", verse_list)
            for verse in verse_list:
                if verse not in detected_verses:
                    detected_verses.append(verse)

    threading.Thread(target=run_stream, daemon=True).start()
    yield  # Let FastAPI run
    # No teardown needed


app = FastAPI(lifespan=lifespan)


@app.get("/", response_class=HTMLResponse)
async def home(request: Request):
    return templates.TemplateResponse(
        "index.html", {"request": request, "verses": detected_verses}
    )
