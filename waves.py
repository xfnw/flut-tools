#!/usr/bin/env python3

from flutrad import *

o = [(500,500,8),(530,500,8),(560,500,8)]

if len(sys.argv) > 1:
    o = eval(sys.argv[1])

m = 128 / len(o)


for y in range(HEIGHT):
    for x in range(WIDTH):
        p = 0
        for k, v, s in o:
            d = sqrt((k - x) ** 2 + (v - y) ** 2)
            p += cos(d/s)
        p = int(p * m + 127) * 65793
        print(f"PX {x} {y} {p:x}")
