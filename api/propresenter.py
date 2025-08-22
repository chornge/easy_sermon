import sys
import asyncio
import json

PRO7_P_HOST = "localhost"
PRO7_P_PORT = 54346


def load_bible(path: str) -> dict:
    with open(path, "r", encoding="utf-8") as f:
        raw = json.load(f)

    bible = {}
    for book in raw["books"]:
        book_name = book["name"]
        bible[book_name] = {}
        for chapter in book["chapters"]:
            chapter_num = str(chapter["chapter"])
            bible[book_name][chapter_num] = {}
            for verse in chapter["verses"]:
                verse_num = str(verse["verse"])
                text = verse["text"].strip()
                bible[book_name][chapter_num][verse_num] = text
    return bible


BIBLE_TEXT = load_bible("american_kjv.json")


# def bible_offline(reference: str) -> str:  # Displays only a single verse
#     try:
#         # Split input like "John 3:16"
#         book, chapter_verse = reference.rsplit(" ", 1)
#         chapter, verse = chapter_verse.split(":")

#         verse_text = BIBLE_TEXT[book][chapter][verse]
#         print(f"{reference} — {verse_text}")
#         return f"{reference} — {verse_text}"
#     except Exception as e:
#         return f"Verse not found: {reference}"


def bible_offline(reference: str) -> str:  # Displays a range of verses
    try:
        book, chapter_verse = reference.rsplit(" ", 1)
        if "-" in chapter_verse:
            chapter, verse_range = chapter_verse.split(":")
            start_verse, end_verse = map(int, verse_range.split("-"))

            lines = []
            for v in range(start_verse, end_verse + 1):
                verse_text = BIBLE_TEXT[book][chapter][str(v)]
                lines.append(f"{book} {chapter}:{v} — {verse_text}")
                print(f"{book} {chapter}:{v} — {verse_text}")
            return "\n".join(lines)
        else:
            chapter, verse = chapter_verse.split(":")
            verse_text = BIBLE_TEXT[book][chapter][verse]
            print(f"{reference} — {verse_text}")
            return f"{reference} — {verse_text}"
    except Exception:
        return f"Verse not found: {reference}"


async def stage_display(verse: str) -> None:
    request_obj = {
        "url": "v1/stage/message",
        "method": "PUT",
        "body": verse,
        "chunked": False,
    }
    request_str = json.dumps(request_obj) + "\r\n"  # CRLF-terminated JSON

    try:
        reader, writer = await asyncio.wait_for(
            asyncio.open_connection(PRO7_P_HOST, PRO7_P_PORT), timeout=3
        )

        writer.write(request_str.encode())
        await writer.drain()
        print(f"✅ {verse} is on Stage Display")

        # 🚫 Don't wait for a response
        writer.close()
        await writer.wait_closed()

    except Exception as e:
        print("❌ Error during send:", e)


if __name__ == "__main__":
    asyncio.run(stage_display(bible_offline("John 3:16")))
    sys.exit(0)
