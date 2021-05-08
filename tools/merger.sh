#!/bin/bash

usage(){
    echo "merger.sh [-l <lib project name>]* [-o <file folder path>] <src project name>"
}

OUTPUT="target"
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

mkdir -p ${OUTPUT}
mkdir -p ${OUTPUT}/src

OUTPUT_SRC="${OUTPUT}/src/main.rs"

echo "/////// AUTOMATICALY - GENERATED  ///////" > "${OUTPUT_SRC}"

for LIB in ${LIBS[@]}
do
    echo "pub mod $LIB{" >> "${OUTPUT_SRC}"
    for file in $(ls ./lib/$LIB/src/*.rs)
    do
        filename=$(basename -- "$file")
        filename="${filename%.*}"
        echo "pub mod $filename{" >> "${OUTPUT_SRC}"
        echo "use super::*;" >> "${OUTPUT_SRC}"
        cat $file >> "${OUTPUT_SRC}"
        echo "}" >> "${OUTPUT_SRC}"
    done
    echo "}" >> "${OUTPUT_SRC}"

    for file in $(ls ./lib/$LIB/src/*.rs)
    do
        filename=$(basename -- "$file")
        filename="${filename%.*}"
        echo "use $LIB::${filename}::*;" >> "${OUTPUT_SRC}"
    done
done


for file in $(ls ./src/${SRC}/src/*.rs)
do
    filename=$(basename -- "$file")
    filename="${filename%.*}"
    if [[ $filename == "main" ]]; then
        cat $file >> "${OUTPUT_SRC}"
    else
        echo "pub mod $filename{" >> "${OUTPUT_SRC}"
        echo "use super::*;" >> "${OUTPUT_SRC}"
        cat $file >> "${OUTPUT_SRC}"
        echo "}" >> "${OUTPUT_SRC}"
    fi
done

for LIB in ${LIBS[@]}
do
    sed -i "s|^use $LIB.*||g" "${OUTPUT_SRC}"
    for file in $(ls ./lib/$LIB/src/*.rs)
    do
        filename=$(basename -- "$file")
        filename="${filename%.*}"
        echo "use $LIB::${filename}::*;" >> "${OUTPUT_SRC}"
    done
done

for file in $(ls ./src/${SRC}/src/*.rs)
do
    filename=$(basename -- "$file")
    filename="${filename%.*}"
    sed -i "s|^use $filename.*||g" "${OUTPUT_SRC}"
    sed -i "s|^mod $filename;||g" "${OUTPUT_SRC}"
done

perl -i -0pe 's|^#\[cfg\(test\)\].*?^}||gms' "${OUTPUT_SRC}"

sed -i "s|^pub mod .*;||g" "${OUTPUT_SRC}"
sed -i "s|^use crate.*;||g" "${OUTPUT_SRC}"

perl -i -0pe 's|\n\n|\n|gms' "${OUTPUT_SRC}"
perl -i -0pe 's|\n\n|\n|gms' "${OUTPUT_SRC}"
perl -i -0pe 's|\n\n|\n|gms' "${OUTPUT_SRC}"


CARGO_PATH="${OUTPUT}/Cargo.toml"
cat <<EOF > $CARGO_PATH
[package]
name = "${SRC}"
version = "1.0.0"
authors = ["Guillaume Escande <escande.guillaume@gmail.com>"]
edition = "2018"

[dependencies]
EOF
