# discord-ext-songbird
[![Test songbird](https://github.com/tuna2134/discord-ext-songbird/actions/workflows/test.yml/badge.svg)](https://github.com/tuna2134/discord-ext-songbird/actions/workflows/test.yml)

Songbird is rust voice manager.

This library is wrapping songbird for discord.py.

[Document](https://tuna2134.dev/discord-ext-songbird/)

## Support
Only macos and linux.

If you are using linux, we are only support this python version.
3.8.10+, 3.9.5+, 3.10.0+

### Why I am not supporting windows?
It's too hard for me.

## Install
```sh
pip install "dextbird @ git+https://github.com/tuna2134/discord-ext-songbird.git"
```

## Sample code
```python
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
        # Play lycoris recoil
        track = await message.guild.voice_client.ytdl("https://youtu.be/Vi-1402wYtI?si=x_rhftnpQ0fKcfEE")
        track.play()
    elif message.content == "!leave":
        await message.guild.voice_client.disconnect()
    elif message.content == "!stop":
        message.guild.voice_client.stop()


client.run(os.getenv("TOKEN"))
```
