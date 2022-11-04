#!/bin/sh

# Read and print lines from stdin.
read_lines() {
    while read -r line; do echo "$line"; done
}
