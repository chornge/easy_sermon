import sys
import asyncio
import json

PRO7_P_HOST = "localhost"
PRO7_P_PORT = 54346


async def stage_display(verse):
    request_obj = {
        "url": "v1/stage/message",
        "method": "PUT",
        "body": verse,
        "chunked": False
    }
    request_str = json.dumps(request_obj) + "\r\n" # CRLF-terminated JSON

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
    asyncio.run(stage_display("Genesis 1:2"))
    sys.exit(0)
