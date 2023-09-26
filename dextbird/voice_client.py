import discord
from .dextbird import Core, Track, Driver

import asyncio
from typing import Optional, Union


class VoiceClient(discord.VoiceProtocol):
    """
    discord.py extensions voiceclient

    Attributes
    ----------
    channel : Union[discord.VoiceChannel, discord.StageChannel]
        Discord VoiceChannel
    client : discord.Client
        Discord.py client instance
    connected : bool
        When the client is connecting to vc, it returns `True`
    """

    channel: Union[discord.VoiceChannel, discord.StageChannel]

    def __init__(self, client: discord.Client, channel: discord.abc.Connectable):
        self._core: Optional[Core] = None
        self.client = client
        self.driver = Driver()
        self.voice_server_event = asyncio.Event()
        self.voice_state_event = asyncio.Event()
        self.connected: bool = False
        super().__init__(client, channel)
        self.guild: discord.Guild = channel.guild

    async def connect(
        self, *, self_deaf: bool = False, self_mute: bool = False, **kwargs
    ) -> None:
        self._core = await Core.setup(self.client, self.guild.id, self.client.user.id)
        await self._core.join(self.channel.id)

        await self.voice_state_event.wait()
        await self.voice_server_event.wait()
        await self._core.connect()
        self.connected = True

    async def on_voice_server_update(self, data: dict) -> None:
        await self._core.update_server(data["endpoint"], data["token"])
        self.voice_server_event.set()

    async def on_voice_state_update(self, data: dict) -> None:
        await self._core.update_state(data["session_id"], data.get("channel_id"))
        self.voice_state_event.set()

    async def ytdl(self, url: str) -> Track:
        """
        Play music by yt-dlp

        Parameters
        ----------
        url : str
            YouTube video url
        """
        return await self._core.ytdl(url)

    def source(self, data: bytes, *, opus: bool = False) -> Track:
        """
        Play music from bytes

        Parameters
        ----------
        data : bytes
            Voice data
        """
        return self._core.source(data, opus)

    def stop(self) -> None:
        "Stop to play music"
        self._core.stop()

    async def disconnect(self, *args) -> None:
        "Disconnect from voice channel"
        await self._core.leave()
        self._core = None
        self.connected = False
        self.cleanup()

    async def deafen(self, deaf: bool) -> None:
        "Deaf connection"
        await self._core.deafen(deaf)

    async def mute(self, mute: bool) -> None:
        "Mute connection"
        await self._core.mute(mute)
