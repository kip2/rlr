#!/bin/bash

ghc -o a.out Main.hs

rlr j "./a.out"

rm ./Main.hi ./Main.o ./a.out

