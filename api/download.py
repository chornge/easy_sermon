from transformers import AutoModelForTokenClassification, AutoTokenizer

# from transformers import DistilBertTokenizer, DistilBertForTokenClassification

# Load a pre-trained model and tokenizer
model_name = "dbmdz/bert-large-cased-finetuned-conll03-english"
# model_name = "dbmdz/distilbert-base-uncased-finetuned-conll03-english"

model = AutoModelForTokenClassification.from_pretrained(model_name)
# model = DistilBertForTokenClassification.from_pretrained(model_name)

tokenizer = AutoTokenizer.from_pretrained(model_name)
# tokenizer = DistilBertTokenizer.from_pretrained(model_name)

# Save the model and tokenizer to a local directory called .model
model.save_pretrained(".model")
tokenizer.save_pretrained(".model")

# try:
#     while True:
#         # Check if we have enough audio data to process (e.g., 1 second)
#         if len(AUDIO_BUFFER) * CHUNK >= RATE:  # 1 second of audio
#             # Concatenate the buffered audio data
#             continue
#         else:
#             continue  # Skip processing
# except KeyboardInterrupt:
#     print("Stopped listening.")
#     print(f"Transcript: {transcript}")
