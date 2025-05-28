from transformers import AutoModelForTokenClassification, AutoTokenizer, pipeline

model = AutoModelForTokenClassification.from_pretrained(".model")

tokenizer = AutoTokenizer.from_pretrained(".model")

nlp = pipeline("ner", model=model, tokenizer=tokenizer)
# nlp = pipeline("ner", model="dbmdz/bert-large-cased-finetuned-conll03-english")

# transcript = "Romans twelve verses one and two"
transcript = "Romans twelve verse three and four"
# transcript = "Genesis 1:2"
# transcript = "Ephesians chapter one and verse five"
# transcript = "Galatians three verse five"
# transcript = "For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life"

results = nlp(transcript)
print(results)

# extracted_verses = [
#     result["word"] for result in results if result["entity"] == "BIBLE"
# ]
# print(extracted_verses)
