#!/usr/bin/env python3
from math import *
import sys

WIDTH=1024
HEIGHT=768

MIDX=WIDTH//2
MIDY=HEIGHT//2
P180=pi/180
P512=pi/512
P1024=pi/1024

def l(v):
    return lambda t: eval(v)

def put(x, y, color='00FF00'):
    print(f"PX {x} {y} {color}")

def graph(func, scale=MIDY//6, color='00FF00'):
    for d in range(1024):
        t = d*P512
        r = func(t)*scale
        x = cos(t)*r
        y = sin(t)*r
        put(int(MIDX+x),int(MIDY-y),color)

if __name__ == '__main__':
    if len(sys.argv) > 1:
        color = sys.argv[1]
    else:
        color = '00FF00'

    if len(sys.argv) > 2:
        bgcolor = sys.argv[2]
    else:
        bgcolor = '000000'

    if len(sys.argv) > 3:
        scale = MIDY//float(sys.argv[3])
    else:
        scale = MIDY//6

    if len(sys.argv) > 4:
        graph(l(sys.argv[4]),color=color, scale=scale)
    else:
        out = lambda t: 0
        while True:
            inp = input()
            graph(out,color=bgcolor, scale=scale)
            out = (lambda t: 0) if len(inp) == 0 else l(inp)
            graph(out,color=color, scale=scale)

