#!/usr/bin/env python3

import asyncio
from asyncio import create_task as ct
from collections import deque


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

    async def drain(self):
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
        y = y % self.height
        x = x % self.width
        return y * self.width + x, x, y

    async def _handle_px(self, params: list):
        x = int(params[0])
        y = int(params[1])
        color = int(params[2], 16)
        pos, x, y = self.topos(x, y)

        self._cache[pos] = color

        if pos in self._waiting:
            self._waiting[pos].set()

    async def get(self, x: int, y: int, cache: bool = True) -> int:
        pos, x, y = self.topos(x, y)

        if not (cache and pos in self._cache):
            if pos in self._waiting:
                if self._waiting[pos].is_set():
                    self._waiting[pos].clear()
                    await self.send(f"PX {x} {y}")
                # else there is currently a pending request
                # for this pixel, no need to send another
            else:
                self._waiting[pos] = asyncio.Event()
                await self.send(f"PX {x} {y}")
            await self._waiting[pos].wait()

        return self._cache[pos]

    async def set(self, x: int, y: int, color: int, cache: bool = True):
        pos, x, y = self.topos(x, y)

        if cache:
            self._cache[pos] = color

        await self.send(f"PX {x} {y} {color:06x}")

    async def clear_cache(self):
        self._cache.clear()

    async def cache_row(self, y: int, step: int = 1):
        pos, x, y = self.topos(0, y * step)

        tosend = deque()
        for x in range(0, self.width, step):
            newpos = pos + x
            if newpos in self._waiting:
                if self._waiting[newpos].is_set():
                    self._waiting[pos].clear()
                    tosend.append(f"PX {x} {y}\n".encode())
            else:
                self._waiting[pos] = asyncio.Event()
                tosend.append(f"PX {x} {y}\n".encode())

        self._writer.writelines(tosend)
        await self._writer.drain()
