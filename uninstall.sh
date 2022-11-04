#!/bin/sh

. ./common.sh

rm "$HOME/.bin/$PROGRAM"
rmdir -p "$HOME/.bin"
rm -rf "$HOME/.$PARTS_FOLDER"
