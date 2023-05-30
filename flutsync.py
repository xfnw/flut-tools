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
        self._connected = asyncio.Event()
        self._cache = {}
        self._waiting = {}

    async def connect(self):
        self._reader, self._writer = await asyncio.open_connection(
            self._host, self._port
        )

        self._connected.set()

    async def send(self, message):
        self._writer.write((message + "\n").encode())
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
                    print("unknown command", unknown)

    async def _handle_size(self, params):
        width = int(params[0])
        height = int(params[1])

        self.width, self.height = width, height

    def topos(self, x, y):
        return (y % self.height) * self.width + (x % self.width)

    async def _handle_px(self, params):
        x = int(params[0])
        y = int(params[1])
        color = int(params[3], 16)
        pos = topos(x, y)

        self._cache[pos] = color

        if pos in self._waiting:
            self._waiting[pos].set()
