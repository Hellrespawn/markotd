#!/bin/sh

PROGRAM="markotd"
PARTS_FOLDER="markotd.d"

mkdir -p "$HOME/.bin"
cp -p markotd.sh "$HOME/.bin/$PROGRAM"

rm -rf "$HOME/.$PARTS_FOLDER"
cp -r "$PARTS_FOLDER" "$HOME/.$PARTS_FOLDER"
