#!/usr/bin/env python3

from flutrad import *

o = [(500,500),(530,500),(560,500)]

if len(sys.argv) > 1:
    o = eval(sys.argv[1])

for k, v in enumerate(o):
    if len(v) == 2:
        o[k] = v + (8,0)
    elif len(v) == 3:
        o[k] = v + (0,)

m = 128 / len(o)


for y in range(HEIGHT):
    for x in range(WIDTH):
        p = 0
        for k, v, s, h in o:
            d = sqrt((k-x) ** 2 + (v-y) ** 2)
            p += cos(d/s + h)
        p = int(p*m + 127) * 65793
        print(f"PX {x} {y} {p:x}")
