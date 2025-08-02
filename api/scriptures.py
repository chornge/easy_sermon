import re
from word2number import w2n
from fuzzywuzzy import process

ORDINALS = {
    "first": "1",
    "second": "2",
    "third": "3",
}

ORDINAL_RULES = {
    "samuel": 2,
    "kings": 2,
    "chronicles": 2,
    "corinthians": 2,
    "thessalonians": 2,
    "timothy": 2,
    "peter": 2,
}

# fmt: off
BIBLE_STRUCTURE = {
    "Genesis": [31,25,24,26,32,22,24,22,29,32,32,20,18,24,21,16,27,33,38,18,34,24,20,67,34,35,46,22,35,43,55,32,20,31,29,43,36,30,23,23,57,38,34,34,28,34,31,22,33,26],
    "Exodus": [22,25,22,31,23,30,29,28,35,29,10,51,22,31,27,36,16,27,25,26,37,30,33,18,40,37,21,43,46,38,18,35,23,35,35,38,29,31,43,38],
    "Leviticus": [17,16,17,35,26,23,38,36,24,20,47,8,59,57,33,34,16,30,33,24,23,55,46,34,34,28,34],
    "Numbers": [54,34,51,49,31,27,89,26,23,36,35,16,33,45,41,35,28,32,22,29,35,41,30,25,19,36,37,27,31,33,33,35,23,34,46,34],
    "Deuteronomy": [46,37,29,49,33,25,26,20,29,22,32,31,19,29,23,22,20,22,21,20,23,29,26,22,19,19,26,68,29,20,30,52,29,12],
    "Joshua": [18,24,17,24,15,27,26,35,27,43,23,24,33,15,63,10,18,28,51,9,45,34,16,33],
    "Judges": [36,23,31,24,31,40,25,35,57,18,40,15,25,20,20,31,13,31,30,48,25],
    "Ruth": [22,23,18,22],
    "1 Samuel": [28,36,21,22,12,21,17,22,24,31,15,25,23,52,35,23,58,30,24,42,15,23,29,22,44,25,12,25,11,31,13],
    "2 Samuel": [27,32,39,12,25,23,29,18,13,19,27,31,39,33,37,23,29,33,43,26,22,51,39,25],
    "1 Kings": [53,46,28,34,18,38,51,66,28,29,43,33,34,31,34,34,24,46,21,43,29,53],
    "2 Kings": [18,25,27,44,27,33,20,29,37,36,21,21,25,29,38,20,41,37,37,21,26,20,37,20,30],
    "1 Chronicles": [54,55,24,43,26,81,40,40,44,14,47,41,14,17,29,43,27,17,19,8,30,19,32,31,31,32,34,21,30],
    "2 Chronicles": [17,18,17,22,14,42,22,18,31,19,23,16,23,14,19,14,19,34,11,37,20,12,21,27,28,23,9,27,36,27,21,33,25,33,27,23],
    "Ezra": [11,70,13,24,17,22,28,36,15,44],
    "Nehemiah": [11,20,32,23,19,19,73,18,38,39,36,47,31],
    "Esther": [22,23,15,17,14,14,10,17,32,3],
    "Job": [22,13,26,21,27,30,21,22,35,22,20,25,28,22,35,22,16,21,29,29,34,30,17,25,6,14,23,28,25,31,40,22,33,37,16,33,24,41,30,24,34,17],
    "Psalm": [6,12,8,8,12,10,17,9,20,18,7,8,6,7,5,11,15,50,14,9,13,31,6,10,22,12,14,9,11,12,24,11,22,22,28,12,40,22,13,17,13,11,5,26,17,11,9,14,20,23,19,9,6,7,23,13,11,11,17,12,8,12,11,10,13,20,7,35,36,5,24,20,28,23,10,12,20,72,13,19,16,8,18,12,13,17,7,18,52,17,16,15,5,23,11,13,12,9,9,5,8,29,22,35,45,48,43,13,31,7,10,10,9,8,18,19,2,29,176,7,8,9,4,8,5,6,5,6,8,8,3,18,3,3,21,26,9,8,24,13,10,7,12,15,21,10,20,14,9,6],
    "Proverbs": [33,22,35,27,23,35,27,36,18,32,31,28,25,35,33,33,28,24,29,30,31,29,35,34,28,28,27,28,27,33,31],
    "Ecclesiastes": [18,26,22,16,20,12,29,17,18,20,10,14],
    "Song Of Solomon": [17,17,11,16,16,12,14,14],
    "Isaiah": [31,22,25,6,30,13,22,22,21,34,16,6,22,32,9,14,14,7,25,6,17,25,18,23,12,21,13,29,24,33,9,20,24,17,10,22,38,22,8,31,29,25,28,28,25,13,15,22,26,11,23,15,12,17,13,12,21,14,21,22,11,12,19,12,25,24,23,23,57,30,34,34,28,34,31,22,44],
    "Jeremiah": [19,37,25,31,31,30,34,22,26,25,23,17,27,22,21,21,27,23,15,18,14,30,40,10,38,24,22,17,32,24,40,44,26,22,19,32,21,28,18,16,18,22,31,7,9,28,23,27,22,17,27,21,23,15,18,14,30,40,10,38,24,22,17,32,24,40,44,26,22,19,32,21,28,18,16,18,22,31,7,9,28,23,27,22],
    "Lamentations": [22,22,66,22,22],
    "Ezekiel": [28,10,27,17,17,14,27,18,11,22,25,28,23,23,9,27,17,22,26,20,27,31,25,24,23,35,40,23,35,27,23,34,16,33,24,23,38,23,29,49,26,20,27,31,25,24,23,35,40,23,35,27,23,34,16,33,24,23,38,23,29],
    "Daniel": [21,49,30,37,31,28,28,27,27,21,45,13],
    "Hosea": [11,23,5,19,15,11,16,14,17,15,12,15,11,17],
    "Joel": [20,32,21],
    "Amos": [15,16,15,13,27,14,17,14,15],
    "Obadiah": [21],
    "Jonah": [17,10,10,11],
    "Micah": [16,13,12,13,15,16,20],
    "Nahum": [15,13,19],
    "Habakkuk": [17,20,19],
    "Zephaniah": [18,15,20],
    "Haggai": [15,23],
    "Zechariah": [17,17,10,14,9,11,16,6,14,10,8,12,14,18],
    "Malachi": [14,17,18,6],
    "Matthew": [25,23,17,25,48,34,29,34,38,42,30,50,58,36,39,28,27,35,30,34,46,46,39,51,46,75,66,20],
    "Mark": [45,28,35,41,43,56,37,38,50,52,33,44,37,72,47,20],
    "Luke": [80,52,38,44,39,49,50,56,62,42,54,59,35,35,32,31,37,43,48,47,38,71,56,53,59,37],
    "John": [51,25,36,54,47,71,53,59,41,42,57,50,38,31,27,33,26,40,42,31,25],
    "Acts": [26,47,26,37,42,15,60,40,43,48,30,25,52,28,41,40,34,28,41,38,40,30,35,27,27,32,44,31],
    "Romans": [32,29,31,25,21,23,25,39,33,21,36,21,14,23,33,27],
    "1 Corinthians": [31,16,23,21,13,20,40,13,27,33,34,31,13,40,58,24],
    "2 Corinthians": [24,17,18,18,21,18,16,24,15,18,33,21,13],
    "Galatians": [24,21,29,31,26,18],
    "Ephesians": [23,22,21,32,33,24],
    "Philippians": [30,30,21,23],
    "Colossians": [29,23,25,18],
    "1 Thessalonians": [10,20,13,18,28],
    "2 Thessalonians": [12,17,18],
    "1 Timothy": [20,15,16,16,25,21],
    "2 Timothy": [18,26,17,22],
    "Titus": [16,15,15],
    "Philemon": [25],
    "Hebrews": [14,18,19,16,14,20,28,13,28,39,40,29,25],
    "James": [27,26,18,17,20],
    "1 Peter": [25,25,22,19,14],
    "2 Peter": [21,22,18],
    "1 John": [10,29,24,21,21],
    "2 John": [13],
    "3 John": [14],
    "Jude": [25],
    "Revelation": [20,29,22,11,14,17,17,13,21,11,19,17,18,20,8,21,18,24,21,15,21,21],
}
# fmt: on

BOOKS = [b.lower() for b in BIBLE_STRUCTURE]
BOOK_PATTERN = r"|".join(sorted(map(re.escape, BOOKS), key=lambda x: -len(x)))

REF_RE = re.compile(
    rf"""
    \b
    (?:(\d+)\s+)?                                        # optional numeric ordinal
    ({BOOK_PATTERN})                                     # book name
    \s+(?:chapter\s+)?([\w\s\-]+?)                       # chapter
    \s+verses?\s+
      ([\w\s\-]+?)                                       # verse start
      (?:\s*(?:-|–|—|to|through|and)\s+([\w\s\-]+?))?    # verse end
    \b
""",
    re.IGNORECASE | re.VERBOSE,
)


def normalize_ordinals(text: str) -> str:
    """Convert 'first'→'1', 'second'→'2', 'third'→'3' everywhere."""
    for word, digit in ORDINALS.items():
        text = re.sub(rf"\b{word}\b", digit, text, flags=re.IGNORECASE)
    return text


def word_to_number(tok: str) -> str:
    if not tok:
        return None
    tok = tok.lower().replace("-", " ").strip()
    try:
        return str(w2n.word_to_num(tok))
    except:
        return tok if tok.isdigit() else None


def fuzzy_book_match(candidate: str) -> str:
    """Return best fuzzy match from BOOKS, or None."""
    match, score = process.extractOne(candidate.lower(), BOOKS)
    return match if score >= 80 else None


def normalize_text(text: str) -> str:
    text = normalize_ordinals(text)
    text = re.sub(r"\bvs\.?\b", "verses", text, flags=re.IGNORECASE)
    text = re.sub(r"\bv\.?\b", "verse", text, flags=re.IGNORECASE)
    return (
        text.replace("psalms", "psalm")
        .replace("revelations", "revelation")
        .replace("songs of solomon", "song of solomon")
    )


def bible_verse(text: str):
    text = normalize_text(text)
    results = []
    for ord_raw, book_raw, chap_raw, verse_start_raw, verse_end_raw in REF_RE.findall(
        text
    ):
        # 1) check ordinal rules
        ord_num = int(ord_raw) if ord_raw else None

        # 2) fuzzy-match book
        fb = fuzzy_book_match(book_raw)
        if not fb:
            continue

        # 3) handle ordinals and special cases
        if fb == "john":
            if ord_num:
                book_key = f"{ord_num} john"  # e.g. "1 john"
            else:
                book_key = "john"
        elif fb in ORDINAL_RULES:
            # ordinal found
            if not ord_num or ord_num > ORDINAL_RULES[fb]:
                continue
            book_key = f"{ord_num} {fb}"
        else:
            # no ordinal needed
            book_key = fb

        book = book_key.title()

        # 4) chapter & verse conversion
        chap = word_to_number(chap_raw)
        start = word_to_number(verse_start_raw)
        end = word_to_number(verse_end_raw)

        if chap is None or start is None:
            continue

        # 5) validate
        if book not in BIBLE_STRUCTURE:
            continue
        chap_n = int(chap)
        if chap_n < 1 or chap_n > len(BIBLE_STRUCTURE[book]):
            continue
        start_n = int(start)
        if start_n < 1 or start_n > BIBLE_STRUCTURE[book][chap_n - 1]:
            continue

        ref = f"{book} {chap_n}:{start_n}"
        if end:
            end_n = int(end)
            if end_n < start_n or end_n > BIBLE_STRUCTURE[book][chap_n - 1]:
                continue
            ref += f"-{end_n}"
        results.append(ref)

    return results


if __name__ == "__main__":
    samples = [
        "at genesis chapter two verses eight and nine",
        "as it says in john three verse sixteen",
        "the book of ezekiel chapter thirty three verse two",
        "going back to psalm one hundred five verse forty",
        "first corinthians thirteen verse four",
        "again in third john one verse two",
        "open your bibles to revelations twenty two verse three",
    ]

    for line in samples:
        print(f"🔍 Audio: {line}")
        print("✅ Got:", bible_verse(line), "\n")
