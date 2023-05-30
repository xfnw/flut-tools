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


async def neighbors(flut, x, y):
    return [
        ON == await flut.get(x - 1, y - 1),
        ON == await flut.get(x, y - 1),
        ON == await flut.get(x + 1, y - 1),
        ON == await flut.get(x - 1, y),
        ON == await flut.get(x + 1, y),
        ON == await flut.get(x - 1, y + 1),
        ON == await flut.get(x, y + 1),
        ON == await flut.get(x + 1, y + 1),
    ].count(True)


async def gol(flut):
    await flut.has_size.wait()

    while True:
        await flut.clear_cache()
        await prereq(flut, -1, 0)
        await prereq(flut, 0, 0)
        await prereq(flut, 1, 0)

        for y in range(flut.height):
            for x in range(flut.width):
                await prereq(flut, x + 2, y)
                num = await neighbors(flut, x, y)

                if await flut.get(x, y) == ON:
                    if not (num == 2 or num == 3):
                        print(f"{x},{y} dies")
                        await flut.set(x, y, OFF)
                else:
                    if num == 3:
                        print(f"{x},{y} is born")
                        await flut.set(x, y, ON)


async def main():
    flut = flutsync.Flutsync(host, port)
    await flut.connect()
    asyncio.create_task(gol(flut))
    await flut.loop()


if __name__ == "__main__":
    asyncio.run(main())
