from dextbird import VoiceClient
import discord
import os


client = discord.Client(intents=discord.Intents.all())


@client.event
async def on_message(message):
    if message.content == "!join":
        await message.author.voice.channel.connect(cls=VoiceClient)


client.run(os.getenv("TOKEN"))
