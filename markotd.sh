#!/bin/sh

. ./common.sh

DIRS="/etc/$PARTS_FOLDER $HOME/.$PARTS_FOLDER"

if [ -n "$DEBUG" ]; then
    DIRS="$DIRS ./$PARTS_FOLDER"
fi

if dir=$(get_dir "$DIRS"); then
    for file in "$dir"/*
    do
        eval "$file"
    done

    exit 0
else
    printf "Unable to find '%s'\n" "$HOME/.$PARTS_FOLDER"
    exit 1
fi

