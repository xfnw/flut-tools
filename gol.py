#!/usr/bin/env python3

import sys, flutsync, asyncio
from asyncio import create_task as ct

from details import host, port

ON = 0x00FF00
OFF = 0x0


async def neighbors(flut, x, y, step=1):
    return [
        ON == await flut.get(x - 1, y - 1, step=step),
        ON == await flut.get(x, y - 1, step=step),
        ON == await flut.get(x + 1, y - 1, step=step),
        ON == await flut.get(x - 1, y, step=step),
        ON == await flut.get(x + 1, y, step=step),
        ON == await flut.get(x - 1, y + 1, step=step),
        ON == await flut.get(x, y + 1, step=step),
        ON == await flut.get(x + 1, y + 1, step=step),
    ].count(True)


async def gol(flut, step=1):
    await flut.has_size.wait()

    iteration = 0

    while True:
        print("iteration", iteration := iteration + 1)

        await flut.clear_cache()
        await flut.cache_row(-1, step=step)
        await flut.cache_row(0, step=step)

        for y in range(flut.height // step):
            # print("row", y)
            await flut.cache_row(y + 1, step=step)
            for x in range(flut.width // step):
                num = await neighbors(flut, x, y)

                if await flut.get(x, y, step=step) == ON:
                    if not (num == 2 or num == 3):
                        # print(f"{x},{y} dies")
                        await flut.set(x, y, OFF, cache=False, step=step)
                else:
                    if num == 3:
                        # print(f"{x},{y} is born")
                        await flut.set(x, y, ON, cache=False, step=step)


async def main():
    step = 1
    if len(sys.argv) > 1:
        step = int(sys.argv[1])

    flut = flutsync.Flutsync(host, port)
    await flut.connect()
    asyncio.create_task(gol(flut, step=step))
    await flut.loop()


if __name__ == "__main__":
    asyncio.run(main())
