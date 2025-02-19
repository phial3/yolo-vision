#!/bin/bash

# env
export FFMPEG_INCLUDE_DIR=/usr/local/ffmpeg/include
export FFMPEG_PKG_CONFIG_PATH=/usr/local/ffmpeg/lib/pkgconfig
export ORT_DYLIB_PATH=/opt/app/onnxruntime-1.20.1/lib/libonnxruntime.1.20.1.dylib

## debug
export RUST_LOG=debug
export RUST_BACKTRACE=full