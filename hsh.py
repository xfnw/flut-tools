#!/usr/bin/env python3

from flutrad import *

color='00FF00'
if len(sys.argv) > 1:
    color=sys.argv[1]
bgcolor='0'
if len(sys.argv) > 2:
    bgcolor=sys.argv[2]

#for i in range(10):
#    graph(lambda x: i,scale=MIDY//5,color='444444')
#
#for i in range(0,WIDTH,2):
#    put(i,MIDY,color='444444')
#for i in range(0,HEIGHT,2):
#    put(MIDX,i,color='444444')

out = lambda t: 0

while True:
    inp = input()
    graph(out, scale=MIDY//2, color=bgcolor)

    out = eval('lambda t: 0'+''.join(['+'+str(1/((i+1)**2))+'*cos('+str((ord(inp[i])-96)/10)+'*t)' for i in range(len(inp))]))

    graph(out, scale=MIDY//2, color=color)
