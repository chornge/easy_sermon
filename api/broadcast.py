active_websockets = set()


def register(ws):
    active_websockets.add(ws)


def unregister(ws):
    active_websockets.discard(ws)


async def broadcast(verse: str):
    for ws in list(active_websockets):
        try:
            await ws.send_text(verse)
        except Exception:
            pass
