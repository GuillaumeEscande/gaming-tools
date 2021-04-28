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

echo "" > main.rs

for LIB in ${LIBS[@]}
do
    cat ./lib/$LIB/src/*.rs >> main.rs
done

cat "${SRC}" >> main.rs

for LIB in ${LIBS[@]}
do
    sed -i "s|use $LIB::$LIB.*||" main.rs
done


perl -i -0pe 's|^#\[cfg\(test\)\].*?^}||gms' main.rs

perl -i -0pe 's|\n\n|\n|gms' main.rs
perl -i -0pe 's|\n\n|\n|gms' main.rs
perl -i -0pe 's|\n\n|\n|gms' main.rs



