import asyncio
import json
import pytest

PRO7_P_HOST = "localhost"
PRO7_P_PORT = 54346

active_websockets = set()


def register(ws):
    active_websockets.add(ws)


def unregister(ws):
    active_websockets.discard(ws)


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


BIBLE_TEXT = load_bible("translations/akjv/akjv.json")


def offline_bible(reference: str) -> str:
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


async def broadcast(verse: str):
    for ws in list(active_websockets):
        try:
            await ws.send_text(verse)
        except Exception:
            pass


def test_offline_bible_with_an_invalid_verse() -> None:
    result = offline_bible("NotABook 1:1")
    expected = "Verse not found: NotABook 1:1"
    assert result == expected


def test_offline_bible_with_a_single_verse() -> None:
    result = offline_bible("2 Corinthians 5:17")
    expected = "2 Corinthians 5:17 — Therefore if any man be in Christ, he is a new creature: old things are passed away; behold, all things are become new."
    assert result == expected


def test_offline_bible_with_a_verse_range() -> None:
    result = offline_bible("Philippians 4:6-7")
    expected = (
        "Philippians 4:6 — Be careful for nothing; but in every thing by prayer and supplication with thanksgiving let your requests be made known to God.\n"
        "Philippians 4:7 — And the peace of God, which passes all understanding, shall keep your hearts and minds through Christ Jesus."
    )
    assert result == expected


@pytest.mark.asyncio
async def test_stage_display(monkeypatch):
    # Mock TCP
    class DummyWriter:
        def write(self, data):
            pass

        async def drain(self):
            pass

        def close(self):
            pass

        async def wait_closed(self):
            pass

    async def dummy_open_connection(host, port):
        return None, DummyWriter()

    monkeypatch.setattr(asyncio, "open_connection", dummy_open_connection)

    # Should not raise any exceptions
    await stage_display("Test verse")


@pytest.mark.asyncio
async def test_broadcast():
    # Mock WebSocket
    class DummyWS:
        def __init__(self):
            self.sent = []

        async def send_text(self, text):
            self.sent.append(text)

    client1 = DummyWS()
    client2 = DummyWS()
    active_websockets.clear()
    active_websockets.add(client1)
    active_websockets.add(client2)

    await broadcast("John 3:16")

    assert client1.sent == ["John 3:16"]
    assert client2.sent == ["John 3:16"]
