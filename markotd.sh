#!/bin/sh

PROGRAM="markotd"
PARTS_FOLDER="markotd.d"
NOTIFY_AFTER_DAYS=3

# Common functions

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

# $1: date parseable by `date`
seconds_since() {
    date1="$(date +%s)"
    date2=$(date -d  "$1" +%s)

    echo "$((date1 - date2))"
}

# $1: date parseable by `date`
hours_since() {
    seconds="$(seconds_since "$1")"
    hours=$((seconds / 60 / 60))

    echo "$hours"
}

# $1: date parseable by `date`
pretty_print_time_since_date() {
    seconds_remaining=$(seconds_since "$1")

    days=$(( seconds_remaining / 60 / 60 / 24 ))
    seconds_remaining=$(( seconds_remaining % (60 * 60 * 24) ))
    hours=$(( seconds_remaining / 60 / 60 ))
    seconds_remaining=$(( seconds_remaining % (60 * 60) ))
    minutes=$(( seconds_remaining / 60 ))
    seconds=$(( seconds_remaining % (60) ))

    date_string="";
    [ "$days" -gt 0 ] && date_string="$date_string, $days days"
    [ "$hours" -gt 0 ] && date_string="$date_string, $hours hours"
    [ "$minutes" -gt 0 ] && date_string="$date_string, $minutes minutes"
    [ "$seconds" -gt 0 ] && date_string="$date_string, $seconds seconds"
    date_string=$(echo "$date_string" | sed 's/^, //' | rev | sed 's/ ,/ dna /' | rev)

    printf 'It has been %s.\n' "$date_string"
}

DIRS="$HOME/.$PARTS_FOLDER"

EMPH=$(tput setaf 6)
RESET=$(tput sgr0)
NOTIFY_AFTER_HOURS="$(( NOTIFY_AFTER_DAYS * 24 ))"

OUTPUT=$(
    if [ -n "$DEBUG" ]; then
        DIRS="./$PARTS_FOLDER $DIRS"
    fi

    if dir=$(get_dir "$DIRS"); then
        for file in "$dir"/*
        do
            # shellcheck source=/dev/null
            . "$file"
        done

    else
        printf "Unable to find '%s'\n" "$HOME/.$PARTS_FOLDER"
        exit 1
    fi
)

echo "$OUTPUT"

