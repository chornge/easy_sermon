import re
from fuzzywuzzy import process
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
    rf"({BOOK_PATTERN})\s+"
    rf"(?:chapter\s+)?(\w+)[\s,:;-]*"
    rf"(?:verse(?:s)?\s+)?(\w+)",
    re.IGNORECASE,
)


def fuzzy_match_book(book_candidate):
    match, score = process.extractOne(book_candidate.lower(), BOOKS)
    return match if score >= 80 else None


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

    # Pass 1: structured regex
    for ordinal, book_base, chapter_raw, verses_raw in matches:
        book_base_fuzzy = fuzzy_match_book(book_base)
        if not book_base_fuzzy:
            continue

        ordinal_num = None
        if ordinal:
            ordinal_num = ORDINALS.get(ordinal.lower(), ordinal)
            try:
                ordinal_int = int(ordinal_num)
            except ValueError:
                continue
            if (
                book_base_fuzzy in ORDINAL_RULES
                and ordinal_int > ORDINAL_RULES[book_base_fuzzy]
            ):
                continue
            book = f"{ordinal_num} {book_base_fuzzy}"
        else:
            if book_base_fuzzy in ORDINAL_RULES and book_base_fuzzy != "john":
                continue
            book = book_base_fuzzy

        book = book.title().replace("Psalms", "Psalm")

        if not verses_raw and chapter_raw:
            parts = chapter_raw.strip().split()
            if len(parts) == 2:
                chapter = word_to_number(parts[0])
                verse_start = word_to_number(parts[1])
                if chapter and verse_start:
                    reference = f"{book} {chapter}:{verse_start}"
                    results.append(reference)
                    continue

        chapter = word_to_number(chapter_raw)
        if not chapter:
            continue

        if not verses_raw:
            reference = f"{book} {chapter}:1"
            results.append(reference)
            continue

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

    # 🆕 Pass 2: Loose fallback — "Book word word" → Book Chapter:Verse
    words = text.lower().split()
    for i in range(len(words) - 2):
        chunk = " ".join(words[i : i + 3])
        match = re.match(r"([a-z]+)\s+([a-z]+)\s+([a-z]+)", chunk)
        if not match:
            continue
        book_candidate, ch_word, v_word = match.groups()
        book_name = fuzzy_match_book(book_candidate)
        if not book_name:
            continue
        chapter = word_to_number(ch_word)
        verse = word_to_number(v_word)
        if chapter and verse:
            reference = f"{book_name.title()} {chapter}:{verse}"
            if reference not in results:
                results.append(reference)

    return results


if __name__ == "__main__":
    samples = [
        "at genesis chapter two verses eight and nine",
        "as it says in john three sixteen",
        "from romans five",
        "ezekiel thirty three verse two",
        "psalms eighty three verse thirty two",
        "open to first corinthians chapter thirteen verse four",
        "psalms one forty three verses two through seven",
        "let's take a look at first corinthians thirteen four",
        "third john one two",
        "second kings seven five",
    ]

    for line in samples:
        result = extract_bible_references(line)
        print("✅ Got:", result)
