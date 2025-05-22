#!/bin/bash

rdmd -of=main main.d

rlr judge "./main"

rm ./main.o
rm ./main

