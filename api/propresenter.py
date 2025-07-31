import aiohttp
import asyncio

PRO7_P_PORT = 54346
PRO7_P_HOST = "localhost"
PASSWORD = ""


async def stage_display(text):
    uri = f"http://{PRO7_P_HOST}:{PRO7_P_PORT}"

    # Send payload
    async with aiohttp.ClientSession().put(
        uri,
        json={
            "url": "v1/stage/message",
            "method": "PUT",
            "body": "Testing new stage message",
            "chunked": "false",
        },
    ) as response:
        if response.status == 200:
            print(f"✅ Sent to Stage Display: {text}")
        else:
            print(f"❌ Failed to send to Stage Display: {response.status}")


if __name__ == "__main__":
    asyncio.run(stage_display("Genesis 1:2"))
    # asyncio.run(stage_display("1 John 1:2"))
