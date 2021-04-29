#!/bin/bash

usage(){
    echo "merger.sh [-l <lib project name>]* [-o <file output path>] <src roject name>"
}

OUTPUT="main.rs"
LIBS=()


while getopts ":o:l:h" option; do
    case "${option}" in
        h)
            usage
            exit 1
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

echo "" > "${OUTPUT}"

for LIB in ${LIBS[@]}
do
    cat ./lib/$LIB/src/*.rs >> "${OUTPUT}"
done

cat ./src/${SRC}/src/*.rs >> "${OUTPUT}"

for LIB in ${LIBS[@]}
do
    sed -i "s|use $LIB::$LIB.*||" "${OUTPUT}"
done


perl -i -0pe 's|^#\[cfg\(test\)\].*?^}||gms' "${OUTPUT}"

perl -i -0pe 's|\n\n|\n|gms' "${OUTPUT}"
perl -i -0pe 's|\n\n|\n|gms' "${OUTPUT}"
perl -i -0pe 's|\n\n|\n|gms' "${OUTPUT}"



