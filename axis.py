#!/usr/bin/env python3

from flutrad import *

if len(sys.argv) > 1:
    for i in range(MIDX+100):
        graph(lambda x: i,scale=1,color='0')

for i in range(1,10):
    graph(lambda x: i,scale=MIDY//5,color='444444')

for i in range(0,WIDTH,2):
    put(i,MIDY,color='444444')
for i in range(0,HEIGHT,2):
    put(MIDX,i,color='444444')

