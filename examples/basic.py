from dextbird import VoiceClient
from discord.ext import commands
import discord

try:
    import dotenv
except ImportError:
    pass
else:
    dotenv.load_dotenv()

import os
import logging


bot = commands.Bot(intents=discord.Intents.all(), command_prefix="!")
logging.getLogger().setLevel(logging.INFO)
        
        
@bot.command()
async def join(ctx: commands.Context) -> None:
    vc = await ctx.author.voice.channel.connect(cls=VoiceClient)
    await ctx.reply("Joined to vc")
    
    
@bot.command()
async def play(ctx: commands.Context, url: str = "https://youtu.be/Vi-1402wYtI") -> None:
    def after():
        print("Play finished")
    track = await ctx.voice_client.ytdl(url)
    track.after(after)
    track.play()
    await ctx.reply("Play some music")

@bot.command()
async def leave(ctx: commands.Context) -> None:
    await ctx.voice_client.disconnect()
    await ctx.reply("Leave from vc")
    
    
@bot.command()
async def stop(ctx: commands.Context) -> None:
    ctx.voice_client.stop()
    await ctx.reply("Stop some music")


@bot.command()
async def ping(ctx: commands.Context) -> None:
    await ctx.reply(
        f"Pong! 🏓\n> {round(bot.latency * 1000, 2)}ms"
    )


bot.run(os.getenv("TOKEN"))
