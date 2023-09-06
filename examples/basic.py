from dextbird import VoiceClient
import discord
import os


client = discord.Client(intents=discord.Intents.all())


@client.event
async def on_message(message):
    if message.content == "!join":
        vc = await message.author.voice.channel.connect(cls=VoiceClient)
        await vc.ytdl("https://www.youtube.com/watch?v=2snqdkxFjt0")


client.run(os.getenv("TOKEN"))