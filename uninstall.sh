#!/bin/sh

PROGRAM="markotd"
PARTS_FOLDER="markotd.d"

rm "$HOME/.bin/$PROGRAM"
rmdir -p "$HOME/.bin"
rm -rf "$HOME/.$PARTS_FOLDER"
