#!/bin/sh

PARTS_FOLDER="./markotd.d"

CWD=$(pwd);

cd "$(dirname "$0")" || exit 1

trap 'cd "$CWD" || exit 1 ' EXIT

for file in "$PARTS_FOLDER"/*
do
    eval "$file"
done

exit 0
