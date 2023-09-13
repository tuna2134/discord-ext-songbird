from typing import Optional, Callable

from discord import Client


async def setup(client: Client, guild_id: int, user_id: int) -> Core:
    "Setup core"


class Core:

    async def join(self, channel_id: int) -> None:
        "Join to vc"
    
    async def connect(self) -> None:
        "Connect to vc gateway"

    async def update_server(self, endpoint: str, token: str) -> None:
        "Update server data"

    async def update_state(self, session_id: str, channel_id: Optional[str]) -> None:
        "Update state"

    async def ytdl(self, url: str) -> Track:
        "Play youtube video's mp3"

    async def source(self, data: bytes, opus: bool) -> Track:
        "Play bytes data"