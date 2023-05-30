#!/usr/bin/env python3

import flutsync,asyncio

async def testme():
    flut = flutsync.Flutsync('raven',6969)
    asyncio.create_task(flut.connect())
    await flut.loop()

asyncio.run(testme())
