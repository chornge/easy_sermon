import sounddevice as sd
import numpy as np
import torch
from transformers import Wav2Vec2ForCTC, Wav2Vec2Processor

# Load the model and processor
model_name = "facebook/wav2vec2-base-960h"
model = Wav2Vec2ForCTC.from_pretrained(model_name)
processor = Wav2Vec2Processor.from_pretrained(model_name)

# Set parameters
RATE = 16000  # Sample rate
CHUNK = 1024  # Number of frames per buffer
AUDIO_BUFFER = []  # Buffer to accumulate audio data


def callback(indata, frames, time, status):
    if status:
        print(status)
    audio_array = indata[:, 0]  # Use the first channel
    AUDIO_BUFFER.append(audio_array)
    # print(f"Received {frames} frames of audio data.")


transcript = ""

# Start the audio stream
with sd.InputStream(callback=callback, channels=1, samplerate=RATE):
    print("Listening...")

    try:
        while True:
            # Check if AUDIO_BUFFER has data before concatenating
            if len(AUDIO_BUFFER) > 0:
                audio_data = np.concatenate(AUDIO_BUFFER)
                AUDIO_BUFFER.clear()  # Clear buffer

                if len(audio_data) < RATE:  # 1 second of audio (not enough audio)
                    continue  # Skip processing

                # Preprocess the audio
                input_values = processor(
                    audio_data,
                    sampling_rate=RATE,
                    return_tensors="pt",
                    padding="longest",
                ).input_values

                # Perform inference
                with torch.no_grad():
                    logits = model(input_values).logits
                print("Inference completed.")

                # Get predicted ids
                predicted_ids = torch.argmax(logits, dim=-1)

                # Decode the ids to text
                transcription = processor.batch_decode(predicted_ids)
                transcript = transcription[0]
                print(f"Transcription: {transcription[0]}")
    except KeyboardInterrupt:
        print(f"Transcript: {transcript}")
