from typing import Optional, Callable

from discord import Client

class Core:
    @staticmethod
    async def setup(client: Client, guild_id: int, user_id: int) -> Core:
        "Return setup"

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

    def source(self, data: bytes) -> Track:
        "Play bytes data"

    async def deafen(self, deaf: bool) -> None:
        "Deaf bot"

    async def mute(self, mute: bool) -> None:
        "Mute bot"

    async def leave(self) -> None:
        "Leave bot from vc"

    def stop(self) -> None:
        "Stop music"

class Track:
    def play(self) -> None:
        "Play some music"

    def set_volume(self, volue: float) -> None:
        "Set client volume"

    def after(self, func: Callable[[], None]) -> None:
        "Set event that call after when the track stop"

    def pause(self) -> None:
        "Pause the track"

    def stop(self) -> None:
        "Stop the track"
