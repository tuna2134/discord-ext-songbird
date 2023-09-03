import discord
from .dextbird import Core, setup


class VoiceClient(discord.VoiceProtocol):

    def __init__(self, client, channel):
        self.channel = channel
        self.guild = channel.guild
        self._core = None
        self.client = client
        # Core(client, channel.guild.id, client.user.id)
        super().__init__()

    async def connect(self, *, self_deaf=False, self_mute=False):
        await setup(self.client, self.guild.id, self.client.user.id)
        await self.guild.change_voice_state(
            self.channel,
            self_deaf=self_deaf,
            self_mute=self_mute,
        )
