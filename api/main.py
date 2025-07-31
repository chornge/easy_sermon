from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.templating import Jinja2Templates

from contextlib import asynccontextmanager
import os
from pathlib import Path
import threading

# import torch

from api.stream import start_vosk_stream, detected_verses

BASE_DIR = Path(__file__).resolve().parent.parent
templates = Jinja2Templates(directory=os.path.join(BASE_DIR, "templates"))

# if torch.cuda.is_available():
#     device = torch.device("cuda")
# elif torch.backends.mps.is_available():
#     device = torch.device("mps")
# else:
#     device = torch.device("cpu")

# if device.type == "cuda" or device.type == "mps":
#     torch.set_default_dtype(torch.float16)
# else:
#     torch.set_default_dtype(torch.float32)

# print(f"Device:", device.type.upper())


@asynccontextmanager
async def lifespan(app: FastAPI):
    threading.Thread(target=start_vosk_stream, daemon=True).start()
    yield


app = FastAPI(lifespan=lifespan)


@app.get("/", response_class=HTMLResponse)
async def home(request: Request):
    return templates.TemplateResponse(
        "index.html", {"request": request, "verses": detected_verses}
    )
