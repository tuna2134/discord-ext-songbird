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
        wait_finished = asyncio.Event()
        def after():
            print("Finished to play music")
            wait_finished.set()
        track = await vc.ytdl("https://youtu.be/_NIp8wvNXmM")
        print("Waiting to finish some music")
        await wait_finished.wait()
        print("Disconnect from vc")
        await vc.disconnect()
        await asyncio.sleep(2)
        print("Finishing test")
        await client.close()

    await client.start(os.getenv("TOKEN"))
