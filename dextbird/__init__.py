import discord
from .dextbird import Core, setup

import asyncio


class VoiceClient(discord.VoiceProtocol):
    def __init__(self, client, channel):
        self.channel = channel
        self.guild = channel.guild
        self._core = None
        self.client = client
        self.voice_server_event = asyncio.Event()
        self.voice_state_event = asyncio.Event()
        self.connected: bool = False
        super().__init__(client, channel)

    async def connect(self, *, self_deaf=False, self_mute=False, **kwargs):
        self._core = await setup(self.client, self.guild.id, self.client.user.id)
        await self._core.join(self.channel.id)

        await self.voice_server_event.wait()
        await self.voice_state_event.wait()
        if not self.connected:
            await self._core.connect()

    async def on_voice_server_update(self, data):
        await self._core.update_server(data["endpoint"], data["token"])
        self.voice_server_event.set()

    async def on_voice_state_update(self, data):
        await self._core.update_state(data["session_id"], data.get("channel_id"))
        self.voice_state_event.set()

    async def ytdl(self, url):
        await self._core.ytdl(url)
