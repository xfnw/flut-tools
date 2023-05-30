#!/usr/bin/env python3

import asyncio
from asyncio import create_task as ct


class Flutsync:
    def __init__(self, host: str, port: int):
        self._host = host
        self._port = port
        self.width = 1024
        self.height = 768
        self.pingtime = 32
        self._reader = None
        self._writer = None
        self._cache = {}
        self._waiting = {}
        self._connected = asyncio.Event()
        self.has_size = asyncio.Event()

    async def connect(self):
        self._reader, self._writer = await asyncio.open_connection(
            self._host, self._port
        )

        self._connected.set()

    async def send(self, message: str):
        self._writer.write((message + "\n").encode())
        await self._writer.drain()

    async def _pingloop(self):
        while True:
            # continually ask for the size, as there is no
            # standard keep-alive command
            await self.send("SIZE")
            await asyncio.sleep(self.pingtime)

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
                    print("unknown command", unknown)

    async def _handle_size(self, params: list):
        width = int(params[0])
        height = int(params[1])

        self.width, self.height = width, height
        self.has_size.set()

    def topos(self, x: int, y: int):
        return (y % self.height) * self.width + (x % self.width)

    async def _handle_px(self, params: list):
        x = int(params[0])
        y = int(params[1])
        color = int(params[3], 16)
        pos = topos(x, y)

        self._cache[pos] = color

        if pos in self._waiting:
            self._waiting[pos].set()

    async def get(self, x: int, y: int, cached: bool=True):
        pass
