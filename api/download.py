import whisper


def download_model(model_name="base.en"):
    print(f"Downloading Whisper model: {model_name}")
    model = whisper.load_model(model_name)
    print("Download complete. Cached at ~/.cache/whisper/{model_name}.pt")
