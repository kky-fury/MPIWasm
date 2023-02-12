#!/bin/bash

if [[ -z "${WASI_SDK_PREFIX}" ]]; then
  echo "You must set WASI_SDK_PREFIX to the location of your download of WASI SDK before running this script"
  exit 1
fi

cmake \
  -DWASI_SDK_PREFIX=$WASI_SDK_PREFIX \
  -DCMAKE_SYSROOT=$WASI_SDK_PREFIX/share/wasi-sysroot \
  -DCMAKE_TOOLCHAIN_FILE=$WASI_SDK_PREFIX/share/cmake/wasi-sdk.cmake
