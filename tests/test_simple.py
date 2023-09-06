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
        channel = client.get_channel(961916734523179050)
        vc = await channel.connect(cls=dextbird.VoiceClient)
        (await vc.ytdl("https://youtu.be/Vi-1402wYtI?si=x_rhftnpQ0fKcfEE")).play()
        await asyncio.sleep(15)
        await client.close()

    await client.start(os.getenv("TOKEN"))
