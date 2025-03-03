#!/bin/bash

# env
export FFMPEG_INCLUDE_DIR=/usr/local/ffmpeg/include
export FFMPEG_PKG_CONFIG_PATH=/usr/local/ffmpeg/lib/pkgconfig
export ORT_DYLIB_PATH=/opt/app/onnxruntime-1.20.1/lib/libonnxruntime.1.20.1.dylib

## debug
export RUST_LOG=debug
export RUST_BACKTRACE=full

## anaconda
# 1. 安装 miniconda
# chmod +x install_miniconda.sh
# ./install_miniconda.sh
# 2. 测试
# conda --version
# 3. 创建 conda 环境
# conda create -n env_test python=3.9 -y
# 4. 激活 conda 环境
# conda activate env_test
# 5. 安装依赖包
# conda install -c conda-forge ffmpeg -y
# 6. 安装 onnxruntime
# pip install onnxruntime
# 7. 验证 onnxruntime
# python -c "import onnxruntime as ort; ort.get_device()"
# 8. 退出 conda 环境
# conda deactivate
# 9. 卸载 conda 的 Python 环境
# conda env remove -n env_test -y
# 10. 卸载 conda
# rm -rf ~/miniconda3
# rm -rf ~/.condarc ~/.conda ~/.continuum