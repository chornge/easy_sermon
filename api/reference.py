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

BOOK_PATTERN = r"|".join(sorted([re.escape(b) for b in BOOKS], key=lambda x: -len(x)))

REFERENCE_PATTERN = re.compile(
    rf"\b(?:(first|second|third|\d(?:st|nd|rd)?)\s+)?"
    rf"({BOOK_PATTERN})"
    rf"(?:\s+chapter)?\s+([\w\s\-]+?)"
    rf"(?:\s+verse(?:s)?\s+([\w\s\-]+?))?"
    rf"(?:\s*(?:-|–|—|to|through|until|and)\s+([\w\s\-]+))?\b",
    re.IGNORECASE,
)


def word_to_number(word):
    if not word:
        return None
    word = word.lower().replace("-", " ").strip()
    try:
        return str(w2n.word_to_num(word))
    except ValueError:
        return word if word.isdigit() else None


def extract_bible_references(text):
    results = []
    for match in REFERENCE_PATTERN.findall(text):
        ordinal, book_base, chapter_raw, verse_start_raw, verse_end_raw = match

        book_base_lower = book_base.lower()
        if ordinal:
            ordinal_num = ORDINALS.get(ordinal.lower(), ordinal)
            try:
                ordinal_int = int(ordinal_num)
                if (
                    book_base_lower not in ORDINAL_RULES
                    or ordinal_int > ORDINAL_RULES[book_base_lower]
                ):
                    continue
                book = f"{ordinal_num} {book_base_lower}"
            except:
                continue
        else:
            if book_base_lower in ORDINAL_RULES and book_base_lower != "john":
                continue
            book = book_base_lower

        book = book.title().replace("Psalms", "Psalm")

        chapter = word_to_number(chapter_raw)
        verse_start = word_to_number(verse_start_raw) if verse_start_raw else "1"
        verse_end = word_to_number(verse_end_raw) if verse_end_raw else None

        if not chapter or not verse_start:
            continue

        reference = f"{book} {chapter}:{verse_start}"
        if verse_end:
            reference += f"-{verse_end}"
        results.append(reference)

    return results


if __name__ == "__main__":
    samples = [
        "at genesis chapter two verses eight and nine",
        "as it says in john three verse sixteen",
        "let's take a look at romans five",
        "ezekiel chapter thirty three verse two",
        "psalms eighty three verse thirty two",
        "psalms one forty three verses two through seven",
        "first corinthians thirteen four",
        "third john one verse two",
        "combining exodus one one leviticus one one and job three one into a module",
        "open your bibles to revelation twenty two verse three",
    ]

    for line in samples:
        print(f"🔍 Audio: {line}")
        print("✅ Got:", extract_bible_references(line), "\n")
