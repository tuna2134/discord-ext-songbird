# discord-ext-songbird

## support
Only macos and linux.

## sample code
```python
from dextbird import VoiceClient
import discord


client = discord.Client(intents=discord.Intents.all())


@client.event
async def on_message(message: discord.Message) -> None:
    if message.content == "!join":
        vc = await message.author.voice.channel.connect(cls=VoiceClient)
        # Play lycoris recoil song
        await vc.ytdl("https://youtu.be/VxR_BYPG7v4")


client.run("TOKEN")
```