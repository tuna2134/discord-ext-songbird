from typing import TypedDict, Optional
import discord


class Option(TypedDict):
    guild_id: int
    channel_id: Optional[int]
    self_deaf: bool
    self_mute: bool


async def change_voice_state(client: discord.Client, option: Option) -> None:
    guild: Optional[discord.Guild] = client.get_guild(option["guild_id"])
    if guild is not None:
        channel = None
        if option["channel_id"] is not None:
            channel = discord.Object(id=Option["channel"])
        await guild.change_voice_state(
            channel, self_deaf=Option["self_deaf"], self_mute=Option["self_mute"]
        )
    else:
        raise Exception("I can't found guild'")
