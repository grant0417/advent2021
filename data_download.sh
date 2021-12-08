#!/bin/sh

# Automatically download the days data
# $1: the number of the day to download
# Cookie is stored in the file "cookie"

cookie="$(< cookie)"

curl \
    --cookie "${cookie}" \
    "https://adventofcode.com/2021/day/${1}/input" > "data/real/day${1}.txt"