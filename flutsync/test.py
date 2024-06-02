#!/usr/bin/env python3

import flutsync, asyncio

from details import host, port


async def getsomepixels(flut):
    await flut.has_size.wait()
    await flut.set(1, 1, 1, cache=False)
    await flut.set(1, 2, 2)
    print(await flut.get(1, 1), await flut.get(1, 2))
    await flut.clear_cache()
    print(await flut.get(1, 1), await flut.get(1, 2))


async def testme():
    flut = flutsync.Flutsync(host, port)
    await flut.connect()
    asyncio.create_task(getsomepixels(flut))
    await flut.loop()


asyncio.run(testme())
