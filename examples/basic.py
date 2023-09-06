from dextbird import VoiceClient
import discord
import os


client = discord.Client(intents=discord.Intents.all())


@client.event
async def on_message(message):
    if message.content == "!join":
        vc = await message.author.voice.channel.connect(cls=VoiceClient)
    elif message.content == "!play":
        await message.guild.voice_client.ytdl("https://www.youtube.com/watch?v=VxR_BYPG7v4")
    elif message.content == "!leave":
        await message.guild.voice_client.disconnect()
    elif message.content == "!stop":
        await message.guild.voice_client.stop()


client.run(os.getenv("TOKEN"))
