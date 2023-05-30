#!/usr/bin/env python3

import asyncio
from asyncio import create_task as ct


class Flutsync:
    def __init__(self, host: str, port: int):
        self._host = host
        self._port = port
        self._reader = None
        self._writer = None
        self._connected = asyncio.Event()
        self.width = 1024
        self.height = 768
        self._cache = {}
        self._waiting = {}
        self.pingtime = 32

    async def connect(self):
        self._reader, self._writer = await asyncio.open_connection(
            self._host, self._port
        )

        self._connected.set()

    async def send(self, message):
        self._writer.write((message+'\n').encode())
        await self._writer.drain()

    async def _pingloop(self):
        while True:
            await asyncio.sleep(self.pingtime)
            await self.send("PING")

    async def loop(self):
        await self._connected.wait()
        ct(self._pingloop())

        while line := await self._reader.readline():
            params = line.split()
            match params.pop(0):
                case b"SIZE":
                    ct(self._handle_size(params))
                case b"PX":
                    ct(self._handle_px(params))
                case unknown:
                    print("unknown command: ", unknown)

    async def _handle_size(self, params):
        pass

    async def _handle_px(self, params):
        pass
