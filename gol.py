#!/usr/bin/env python3

import flutsync, asyncio
from asyncio import create_task as ct

from details import host, port

ON = 0x00FF00
OFF = 0x0


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
        await flut.cache_row(-1)
        await flut.cache_row(0)

        for y in range(flut.height):
            print("row", y)
            await flut.cache_row(y + 1)
            for x in range(flut.width):
                num = await neighbors(flut, x, y)

                if await flut.get(x, y) == ON:
                    if not (num == 2 or num == 3):
                        print(f"{x},{y} dies")
                        await flut.set(x, y, OFF, cache=False)
                else:
                    if num == 3:
                        print(f"{x},{y} is born")
                        await flut.set(x, y, ON, cache=False)


async def main():
    flut = flutsync.Flutsync(host, port)
    await flut.connect()
    asyncio.create_task(gol(flut))
    await flut.loop()


if __name__ == "__main__":
    asyncio.run(main())
