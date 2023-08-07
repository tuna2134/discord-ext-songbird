import discord
from .dextbird import Core


class VoiceClient(discord.VoiceProtocol):

    def __init__(self, client, channel):
        self.channel = channel
        self.guild = channel.guild
        self._core = Core(client, channel.guild.id, client.user.id)
        super().__init__()

    async def connect(self, *, self_deaf=False, self_mute=False):
        await self.guild.change_voice_state(
            self.channel,
            self_deaf=self_deaf,
            self_mute=self_mute,
        )