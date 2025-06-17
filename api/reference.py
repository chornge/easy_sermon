import re
from word2number import w2n

ORDINALS = {
    "first": "1",
    "second": "2",
    "third": "3",
    "1st": "1",
    "2nd": "2",
    "3rd": "3",
}

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

BOOK_PATTERN = r"|".join(re.escape(book) for book in BOOKS)

REFERENCE_PATTERN = re.compile(
    rf"\b(?:(first|second|third|\d(?:st|nd|rd)?)\s+)?"
    rf"({BOOK_PATTERN})[\s,]+"
    rf"(?:chapter\s+)?([\w\s\-]+?)\s*"
    rf"(?:verse(?:s)?|vs|v|versus)?\s*([\w\s\-]+)?",
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


def normalize_transcription(text: str) -> str:
    replacements = {
        "vs.": "verse",
        "vs": "verse",
        "v.": "verse",
        "v ": "verse ",
        "versus": "verse",
        ",": "",
    }
    for wrong, correct in replacements.items():
        text = text.replace(wrong, correct)
    return text.strip()


def extract_bible_references(text):
    text = normalize_transcription(text)
    print("🔍 Extracting Bible references from:", text)
    matches = REFERENCE_PATTERN.findall(text)
    results = []

    for ordinal, book_base, chapter_raw, verses_raw in matches:
        book_base_lower = book_base.lower()
        ordinal_num = None

        if ordinal:
            ordinal_num = ORDINALS.get(ordinal.lower(), ordinal)
            try:
                ordinal_int = int(ordinal_num)
            except ValueError:
                continue
            if (
                book_base_lower in ORDINAL_RULES
                and ordinal_int > ORDINAL_RULES[book_base_lower]
            ):
                continue
            book = f"{ordinal_num} {book_base_lower}"
        else:
            if book_base_lower in ORDINAL_RULES and book_base_lower != "john":
                continue
            book = book_base_lower

        book = book.title().replace("Psalms", "Psalm")

        chapter = word_to_number(chapter_raw)
        if not chapter:
            continue

        if not verses_raw:
            # Only chapter mentioned → default to verse 1
            verse_start = "1"
            reference = f"{book} {chapter}:{verse_start}"
            results.append(reference)
            continue

        # Handle "3 and 4", "4 through 6", "7 to 9", etc.
        parts = re.split(r"\s*(?:and|to|through|thru|until|-|–|—)\s*", verses_raw)
        if len(parts) == 1:
            verse_start = word_to_number(parts[0])
            if not verse_start:
                continue
            reference = f"{book} {chapter}:{verse_start}"
        elif len(parts) == 2:
            verse_start = word_to_number(parts[0])
            verse_end = word_to_number(parts[1])
            if not verse_start or not verse_end:
                continue
            reference = f"{book} {chapter}:{verse_start}-{verse_end}"
        else:
            continue

        results.append(reference)

    return results
