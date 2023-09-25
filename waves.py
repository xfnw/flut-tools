#!/usr/bin/env python3

from flutrad import *

o = [(500,500),(530,500),(560,500)]

m = 128 / len(o)

for y in range(HEIGHT):
    for x in range(WIDTH):
        p = 0
        for k, v in o:
            d = sqrt((k - x) ** 2 + (v - y) ** 2)
            p += cos(d/8)
        p = int(p * m + 127) * 65793
        print(f"PX {x} {y} {p:x}")
