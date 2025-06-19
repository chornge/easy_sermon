from io import BytesIO
import requests
import zipfile

url = "https://alphacephei.com/vosk/models/vosk-model-en-us-0.22.zip"  # 2.7GB | 99.5% accurate
# url = "https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip" #68 MB | 95% accurate

resp = requests.get(url)
z = zipfile.ZipFile(BytesIO(resp.content))
z.extractall("models/")
