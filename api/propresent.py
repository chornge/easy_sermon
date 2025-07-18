import asyncio
import json
import websockets

PRO7_HOST = "localhost/v1/stage/message"  # PUT instead of POST
PRO7_PORT = 1025  # Set ProPresenter port
PASSWORD = ""  # Set ProPresenter password (env)


async def send_text_to_propresenter(text):
    uri = f"ws://{PRO7_HOST}:{PRO7_PORT}/remote"

    async with websockets.connect(uri) as websocket:
        await websocket.send(
            json.dumps(
                {"action": "authenticate", "protocol": 701, "password": PASSWORD}
            )
        )
        auth_response = await websocket.recv()
        print("✅ Auth Response:", auth_response)

        # Send text to clear message layer (as a "Message")
        await websocket.send(json.dumps({"action": "message", "text": text}))

        response = await websocket.recv()
        print("📤 Sent:", response)


if __name__ == "__main__":
    asyncio.run(send_text_to_propresenter("Genesis 1:2"))
    # asyncio.run(send_text_to_propresenter("1 John 1:2"))
