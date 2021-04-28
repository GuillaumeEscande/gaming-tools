#!/bin/bash

usage(){
    echo "usage"
}

OUTPUT=""
LIBS=()
VERBOSE=false


while getopts ":o:l:hv" option; do
    case "${option}" in
        h)
            usage
            exit 1
            ;;
        v)
            VERBOSE=true
            ;;
        o)
            OUTPUT="${OPTARG}"
            ;;
        l)
            LIBS+=("${OPTARG}")
            ;;
        *)
            usage
            ;;
    esac
done
shift $((OPTIND-1))

SRC="$@"

echo "OUTPUT=${OUTPUT}"
echo "SRC=${SRC}"
echo "LIBS=${LIBS[@]}"
echo "VERBOSE=${VERBOSE}"

for f in "a.rs" "b.rs" "c.rs"
do
    echo "mod ${f%.*} {" >> main.rs
    cat "$f" >> main.rs
    echo "}" >> main.rs
done