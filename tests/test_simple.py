import discord
import pytest
import dextbird

try:
    import dotenv
except ImportError:
    pass
else:
    dotenv.load_dotenv()

import asyncio
import os
import logging
import random


MUSICS = [
    "https://youtu.be/fE9trKOuT3Q",
    "https://youtu.be/TG2IgWOjtwU",
    "https://youtu.be/yL1LYf-S2Q0"
]


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
        logging.info("Deafen")
        track = await vc.ytdl(random.choice(MUSICS))
        logging.info("Play youtube")
        track.after(after)
        track.play()
        logger.info("Waiting to finish some music")
        try:
            await asyncio.wait_for(wait_finished.wait(), timeout=60 * 5)
        except asyncio.TimeoutError:
            logger.error("Timeout to wait playing music")
        logger.info("Disconnect from vc")
        await vc.disconnect()
        await asyncio.sleep(2)
        logger.info("Finishing test")
        await client.close()

    async with client:
        await client.start(os.getenv("DISCORD_TOKEN"))
