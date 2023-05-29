#!/usr/bin/env python3

import asyncio

from exceptions import CannotBeReused


class Flutsync:
    def __init__(self, host: str, port: int):
        self._already = False
        self._host = host
        self._port = port
        self.width = 1024
        self.height = 768
        self._cache = {}
        self._waiting = {}
        self.ttl = 32

    async def connect(self):
        if self._already:
            raise CannotBeReused
        self._already = true

        asyncio.create_task(self._readerloop())
