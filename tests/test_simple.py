import discord
import pytest
import dextbird
import asyncio
import os


client = discord.Client(intents=discord.Intents.all())


@pytest.mark.asyncio
async def test_some_asyncio_code():
    @client.event
    async def on_ready() -> None:
        print("Starting test")
        channel = client.get_channel(961916734523179050)
        print("Connecting to vc")
        vc = await channel.connect(cls=dextbird.VoiceClient)
        print("Playing music")
        (await vc.ytdl("https://youtu.be/_NIp8wvNXmM")).play()
        print("Waiting 105 second")
        await asyncio.sleep(105)
        print("Disconnect from vc")
        await vc.disconnect()
        await asyncio.sleep(2)
        print("Finishing test")
        await client.close()

    await client.start(os.getenv("TOKEN"))
