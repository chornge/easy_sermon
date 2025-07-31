import aiohttp
import asyncio

PRO7_PORT = 49279
PRO7_HOST = "localhost"
PASSWORD = ""


async def propresenter(text):
    uri = f"http://{PRO7_HOST}:{PRO7_PORT}/v1/stage/message"

    # Prepare the message payload
    payload = {
        "Found": text,
    }

    # Send payload to ProPresenter
    async with aiohttp.ClientSession() as session:
        async with session.put(uri, json=payload) as response:
            if response.status == 200:
                print(f"✅ Sent to ProPresenter: {text}")
            else:
                print(f"❌ Failed to send to ProPresenter: {response.status}")


if __name__ == "__main__":
    asyncio.run(propresenter("Genesis 1:2"))
    # asyncio.run(propresenter("1 John 1:2"))
