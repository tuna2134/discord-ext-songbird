import discord
import pytest
import dextbird

import asyncio
import os
import logging


client = discord.Client(intents=discord.Intents.all())
logger = logging.getLogger()


@pytest.mark.asyncio
async def test_some_asyncio_code():
    @client.event
    async def on_ready() -> None:
        logger.info("Starting test")
        channel = client.get_channel(961916734523179050)
        logger.info("Connecting to vc")
        vc = await channel.connect(cls=dextbird.VoiceClient)
        logger.info("Playing music")
        wait_finished = asyncio.Event()
        def after():
            logger.info("Finished to play music")
            wait_finished.set()
        await vc.deafen(True)
        track = await vc.ytdl("https://youtu.be/fE9trKOuT3Q")
        track.after(after)
        track.play()
        logger.info("Waiting to finish some music")
        await wait_finished.wait()
        logger.info("Disconnect from vc")
        await vc.disconnect()
        await asyncio.sleep(2)
        logger.info("Finishing test")
        await client.close()

    async with client:
        await client.start(os.getenv("TOKEN"))
