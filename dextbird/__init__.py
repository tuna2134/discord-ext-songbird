import discord
from .dextbird import Core


class VoiceClient(VoiceProtocol):

    def __init__(self, client, channel):
        self._core = Core(client, channel.guild, client.user.id)

    async def connect(self, **kwargs):
        await self._core.join()
