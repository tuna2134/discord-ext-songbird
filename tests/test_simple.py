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
        (await vc.ytdl("https://youtu.be/Vi-1402wYtI?si=x_rhftnpQ0fKcfEE")).play()
        print("Waiting 240 second")
        await asyncio.sleep(240)
        print("Disconnect from vc")
        await vc.disconnect()
        print("Finishing test")
        await client.close()

    await client.start(os.getenv("TOKEN"))
