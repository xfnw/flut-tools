#!/usr/bin/env python3

import flutsync,asyncio

async def getsomepixels(flut):
    await flut.has_size.wait()
    await flut.set(1,1,1,cache=False)
    await flut.set(1,2,1)
    print(await flut.get(1,1), await flut.get(1,2))

async def testme():
    flut = flutsync.Flutsync('raven',6969)
    asyncio.create_task(flut.connect())
    asyncio.create_task(getsomepixels(flut))
    await flut.loop()

asyncio.run(testme())
