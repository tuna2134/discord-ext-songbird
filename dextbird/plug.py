from typing import TypedDict, Optional
import discord


class Option(TypedDict):
    guild_id: int
    channel_id: Optional[int]
    self_deaf: bool
    self_mute: bool


async def update_voice_state(client: discord.Client, option: Option) -> None:
    guild: Optional[discord.Guild] = client.get_guild(option["guild_id"])
    if guild is not None:
        channel = None
        if option["channel_id"] is not None:
            channel = discord.Object(id=option["channel_id"])
        await guild.change_voice_state(
            channel=channel, self_deaf=option["self_deaf"], self_mute=option["self_mute"]
        )
    else:
        raise Exception("I can't found guild'")
