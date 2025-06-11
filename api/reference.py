import re

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

# Build a regex pattern for book names
BOOK_PATTERN = r"|".join(re.escape(book) for book in BOOKS)

# Full pattern with optional ordinal prefix, chapter, verse(s), and span
REFERENCE_PATTERN = re.compile(
    rf"\b(?:(first|second|third|\d(?:st|nd|rd)?)\s+)?"
    rf"({BOOK_PATTERN})\s+"
    rf"(?:chapter\s+)?(\d+)[\s,:;-]*"
    rf"(?:verse(?:s)?\s+)?(\d+)"
    rf"(?:\s*(?:-|–|—|to|through)\s*(\d+))?",
    re.IGNORECASE,
)


def extract_bible_references(text):
    matches = REFERENCE_PATTERN.findall(text)
    results = []

    for match in matches:
        ordinal, book, chapter, verse_start, verse_end = match

        # Normalize ordinal prefix
        if ordinal:
            ordinal = ORDINALS.get(ordinal.lower(), ordinal)
            book = f"{ordinal} {book}"
        book = book.title().replace("Psalms", "Psalm")  # Normalize plural

        # Format reference
        if verse_end:
            reference = f"{book} {chapter}:{verse_start}-{verse_end}"
        else:
            reference = f"{book} {chapter}:{verse_start}"

        results.append(reference)

    return results
