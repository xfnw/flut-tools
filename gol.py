#!/usr/bin/env python3

import flutsync, asyncio
from asyncio import create_task as ct

from details import host, port

ON = 0x00FF00
OFF = 0x0


async def prereq(flut, x, y):
    """
    request the data we need all at once, rather having to
    block for sequential responses to each request later
    """
    ct(flut.get(x, y - 1))
    ct(flut.get(x, y))
    ct(flut.get(x, y + 1))


async def gol(flut):
    await flut.has_size.wait()
    await prereq(flut, 0, 0)


async def main():
    flut = flutsync.Flutsync(host, port)
    await flut.connect()
    asyncio.create_task(gol(flut))
    await flut.loop()


if __name__ == "__main__":
    asyncio.run(main())
