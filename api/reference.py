import re
from num2words import num2words

ORDINALS = {"first": "1", "second": "2", "third": "3"}

# List of Bible books (canonical order, lowercase for matching)
BOOKS = [
    "genesis",
    "exodus",
    "leviticus",
    "numbers",
    "deuteronomy",
    "joshua",
    "judges",
    "ruth",
    "1 samuel",
    "2 samuel",
    "1 kings",
    "2 kings",
    "1 chronicles",
    "2 chronicles",
    "ezra",
    "nehemiah",
    "esther",
    "job",
    "psalm",
    "psalms",
    "proverbs",
    "ecclesiastes",
    "song of solomon",
    "isaiah",
    "jeremiah",
    "lamentations",
    "ezekiel",
    "daniel",
    "hosea",
    "joel",
    "amos",
    "obadiah",
    "jonah",
    "micah",
    "nahum",
    "habakkuk",
    "zephaniah",
    "haggai",
    "zechariah",
    "malachi",
    "matthew",
    "mark",
    "luke",
    "john",
    "acts",
    "romans",
    "1 corinthians",
    "2 corinthians",
    "galatians",
    "ephesians",
    "philippians",
    "colossians",
    "1 thessalonians",
    "2 thessalonians",
    "1 timothy",
    "2 timothy",
    "titus",
    "philemon",
    "hebrews",
    "james",
    "1 peter",
    "2 peter",
    "1 john",
    "2 john",
    "3 john",
    "jude",
    "revelation",
]

# Generate number words up to 176 ("one hundred seventy-six" becomes "176")
NUMBER_WORDS = {num2words(i).replace("-", " "): str(i) for i in range(1, 177)}

# Build a regex pattern for book names
BOOK_PATTERN = r"|".join(re.escape(book) for book in BOOKS)

# Full pattern with optional ordinal prefix, chapter, verse(s), and span
REFERENCE_PATTERN = re.compile(
    rf"\b(?:(first|second|third|\d(?:st|nd|rd)?)\s+)?"
    rf"({BOOK_PATTERN})\s+"
    rf"(?:chapter\s+)?(\w+)[\s,:;-]*"
    rf"(?:verse(?:s)?\s+)?(\w+)"
    rf"(?:\s*(?:-|–|—|to|through)\s*(\w+))?",
    re.IGNORECASE,
)


def word_to_number(word):
    word = word.lower().replace("-", " ")
    if word.isdigit():
        return word
    return NUMBER_WORDS.get(word)


def extract_bible_references(text):
    matches = REFERENCE_PATTERN.findall(text)
    results = []

    for ordinal, book, chapter, verse_start, verse_end in matches:
        if ordinal:
            ordinal = ORDINALS.get(ordinal.lower(), ordinal)
            book = f"{ordinal} {book}"
        book = book.title().replace("Psalms", "Psalm")  # Normalize plural

        chapter = word_to_number(chapter)
        verse_start = word_to_number(verse_start)
        verse_end = word_to_number(verse_end) if verse_end else None

        if not chapter or not verse_start:
            continue

        reference = f"{book} {chapter}:{verse_start}"
        if verse_end:
            reference += f"-{verse_end}"

        results.append(reference)

    return results
