#!/bin/bash
set -xeu

ARCH=${1}
NAME=${2}
echo "Installing cross..."
cargo install cross --git https://github.com/cross-rs/cross
echo "Building for arch ${ARCH}..."
cross build --release --target ${ARCH}-unknown-linux-gnu
echo "Copying bin file..."
OUT_FOLDER=$NAME
BIN_FOLDER="target/${ARCH}-unknown-linux-gnu/release"
mkdir $OUT_FOLDER
cp $BIN_FOLDER/$NAME $OUT_FOLDER/
cp -r models $OUT_FOLDER/
cp -r $BIN_FOLDER/build/pv_porcupine-*/out/lib $OUT_FOLDER/models/libpv_porcupine
cp -r $BIN_FOLDER/build/pv_recorder-*/out/lib $OUT_FOLDER/models/libpv_recorder
cd $OUT_FOLDER
tar czf ../${NAME}_${ARCH}.tar.gz *