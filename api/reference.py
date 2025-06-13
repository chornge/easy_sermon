import re

from num2words import num2words
from word2number import w2n

ORDINALS = {
    "first": "1",
    "second": "2",
    "third": "3",
    "1st": "1",
    "2nd": "2",
    "3rd": "3",
}

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

# Books and their valid ordinals (e.g., "john" can be 1st or 2nd or 3rd)
ORDINAL_RULES = {
    "john": 3,
    "peter": 2,
    "timothy": 2,
    "thessalonians": 2,
    "corinthians": 2,
    "kings": 2,
    "samuel": 2,
    "chronicles": 2,
}

# Accept number words like "one hundred twenty" up to 176
NUMBER_WORDS = {num2words(i).replace("-", " "): str(i) for i in range(1, 177)}

# Regex pattern for book names
BOOK_PATTERN = r"|".join(re.escape(book) for book in BOOKS)

# Regex pattern to match spoken references
REFERENCE_PATTERN = re.compile(
    rf"\b(?:(first|second|third|\d(?:st|nd|rd)?)\s+)?"
    rf"({BOOK_PATTERN})\s+"
    rf"(?:chapter\s+)?([\w\s\-]+?)[\s,:;-]*"
    rf"(?:verse(?:s)?\s+)?([\w\s\-]+)?"
    rf"(?:\s*(?:-|–|—|to|through)\s*([\w\s\-]+))?",
    re.IGNORECASE,
)


def word_to_number(word):
    if not word:
        return None
    word = word.lower().replace("-", " ").strip()
    try:
        return str(w2n.word_to_num(word))
    except ValueError:
        if word.isdigit():
            return word
        return None


def extract_bible_references(text):
    matches = REFERENCE_PATTERN.findall(text)
    results = []

    for ordinal, book_base, chapter_raw, verse_start_raw, verse_end_raw in matches:
        book_base_lower = book_base.lower()
        ordinal_num = None

        if ordinal:
            ordinal_num = ORDINALS.get(ordinal.lower(), ordinal)
            try:
                ordinal_int = int(ordinal_num)
            except ValueError:
                continue

            # Validate ordinal based on rules
            max_valid = ORDINAL_RULES.get(book_base_lower)
            if not max_valid or ordinal_int > max_valid:
                print(f"⛔ Invalid ordinal for book: {ordinal} {book_base}")
                continue
            book = f"{ordinal_num} {book_base_lower}"
        else:
            if book_base_lower in ORDINAL_RULES:
                # Only allow unprefixed 'john'
                if book_base_lower != "john":
                    print(f"⛔ Missing ordinal for book: {book_base}")
                    continue
                book = "john"
            else:
                book = book_base_lower

        # Capitalize book properly
        book = book.title().replace("Psalms", "Psalm")

        chapter = word_to_number(chapter_raw)
        verse_start = word_to_number(verse_start_raw)
        verse_end = word_to_number(verse_end_raw) if verse_end_raw else None

        if not chapter or not verse_start:
            continue

        reference = f"{book} {chapter}:{verse_start}"
        if verse_end:
            reference += f"-{verse_end}"
        results.append(reference)

    return results
