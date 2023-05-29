#!/usr/bin/env python3

import asyncio
from collections import deque

class Flutsync():
    def __init__(self, host: str, port: int):
        self._first = False
        self._host = host
        self._port = port
        self._cache = {}
        self._queue = deque()
        self.ttl = 32


