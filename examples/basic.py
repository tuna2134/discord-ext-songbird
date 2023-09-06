from dextbird import VoiceClient
import discord

import os
import logging


client = discord.Client(intents=discord.Intents.all())
logging.getLogger().setLevel(logging.INFO)


@client.event
async def on_message(message):
    if message.content == "!join":
        vc = await message.author.voice.channel.connect(cls=VoiceClient)
    elif message.content == "!play":
        def after():
            print("Played")
        track = await message.guild.voice_client.ytdl("https://youtu.be/Vi-1402wYtI?si=x_rhftnpQ0fKcfEE")
        track.after(after)
        track.play()
    elif message.content == "!leave":
        await message.guild.voice_client.disconnect()
    elif message.content == "!stop":
        await message.guild.voice_client.stop()


client.run(os.getenv("TOKEN"))
