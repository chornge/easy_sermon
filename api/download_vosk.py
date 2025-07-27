from io import BytesIO
import requests
import zipfile

# Download model (ranked by accuracy)
url = "https://alphacephei.com/vosk/models/vosk-model-en-us-0.42-gigaspeech.zip"  # 4.1 GB | 94.36% accurate
# url = "https://alphacephei.com/vosk/models/vosk-model-en-us-0.22.zip"           # 2.9 GB | 94.31% accurate
# url = "https://alphacephei.com/vosk/models/vosk-model-en-us-0.22-lgraph.zip"    # 215 MB | 92.18% accurate
# url = "https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip"     # 71 MB  | 90.15% accurate

resp = requests.get(url)
z = zipfile.ZipFile(BytesIO(resp.content))
z.extractall("models/")
