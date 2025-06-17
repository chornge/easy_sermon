import whisper
import sounddevice as sd
import tempfile
import scipy.io.wavfile
import warnings

from api.reference import extract_bible_references

model = whisper.load_model("small.en")

# Config Settings
duration = 5  # seconds per chunk
sample_rate = 16000
warnings.filterwarnings("ignore", message="FP16 is not supported on CPU*")


def stream_bible_verses():
    try:
        while True:
            audio_chunk = record_audio(duration, sample_rate)
            text = transcribe_audio(audio_chunk)
            references = extract_bible_references(text.strip())
            print(f"Listening in {duration}s increments. Press Ctrl+C to stop.")
            if references:
                yield references
    except KeyboardInterrupt:
        print("\n🛑 Program stopped by user.")
    finally:
        sd.stop()


def record_audio(duration, sample_rate):
    # sd.default.device = ("iMac Microphone", "iMac Speakers") # Input, Output
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


if __name__ == "__main__":
    try:
        while True:
            audio_chunk = record_audio(duration, sample_rate)
            text = transcribe_audio(audio_chunk)
            print(">>", extract_bible_references(text.strip()))
    except KeyboardInterrupt:
        print("\nProgram has stopped.")
