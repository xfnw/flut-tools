#!/usr/bin/env python3

from flutrad import *

bgcolor='0'
if len(sys.argv) > 1:
    bgcolor=sys.argv[1]

def mew(x):
  return lambda t: sin(5*t+x)+cos(3*t+x)

i=0
while True:
  graph(mew(i),scale=MIDY//2,color='ff4d00')
  graph(mew(i-1),scale=MIDY//2,color=bgcolor)
  i+=1


