#!/bin/sh

PROGRAM="markotd"
PARTS_FOLDER="markotd.d"

# Read and print lines from stdin.
read_lines() {
    while read -r line; do echo "$line"; done
}

# $1: Whitespace-separated list of filenames
get_file() {
    for filename in $1; do
        if [ -f "$filename" ]; then
            echo "$filename"
            return 0
        fi
    done

    return 1
}


# $1: Whitespace-separated list of filenames
get_dir() {
    for dirname in $1; do
        if [ -d "$dirname" ]; then
            echo "$dirname"
            return 0
        fi
    done

    return 1
}
