import discord
from .dextbird import Core, setup, Track

import asyncio
from typing import Optional


class VoiceClient(discord.VoiceProtocol):
    "discord.py extensions voiceclient"
    
    def __init__(self, client: discord.Client, channel: discord.abc.Connectable):
        self.channel = channel
        self.guild: discord.Guild = channel.guild
        self._core: Optional[Core] = None
        self.client = client
        self.voice_server_event = asyncio.Event()
        self.voice_state_event = asyncio.Event()
        self.connected: bool = False
        super().__init__(client, channel)

    async def connect(self, *, self_deaf: bool=False, self_mute: bool=False, **kwargs) -> None:
        "Connect to voice channel"
        self._core = await setup(self.client, self.guild.id, self.client.user.id)
        await self._core.join(self.channel.id)

        await self.voice_state_event.wait()
        await self.voice_server_event.wait()
        await self._core.connect()
        self.connected = True

    async def on_voice_server_update(self, data: dict) -> None:
        "Update voice server"
        await self._core.update_server(data["endpoint"], data["token"])
        self.voice_server_event.set()

    async def on_voice_state_update(self, data: dict) -> None:
        "Update voice state"
        await self._core.update_state(data["session_id"], data.get("channel_id"))
        self.voice_state_event.set()

    async def ytdl(self, url: str) -> Track:
        "Play music by yt-dlp"
        return await self._core.ytdl(url)

    async def source(self, data: bytes, *, opus: bool=True) -> Track:
        "Play music from bytes"
        return await self._core.source(data, opus)

    async def stop(self) -> None:
        "Stop to play music"
        await self._core.stop()

    async def disconnect(self, *args) -> None:
        "Disconnect from voice channel"
        await self._core.leave()
        self._core = None
        self.connected = False
        self.cleanup()

    async def deafen(self, deaf: bool) -> None:
        "Deaf connection"
        await self._core.deafen(deaf)
