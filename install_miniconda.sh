#!/bin/bash

# 下载 Miniconda
cd ~
wget https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh

# 安装 Miniconda
bash Miniconda3-latest-Linux-x86_64.sh -b -p $HOME/miniconda3

# 配置环境变量
echo 'export PATH="$HOME/miniconda3/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# 初始化 conda
$HOME/miniconda3/bin/conda init bash

# 配置清华源
$HOME/miniconda3/bin/conda config --add channels https://mirrors.tuna.tsinghua.edu.cn/anaconda/pkgs/free/
$HOME/miniconda3/bin/conda config --add channels https://mirrors.tuna.tsinghua.edu.cn/anaconda/pkgs/main/
$HOME/miniconda3/bin/conda config --add channels https://mirrors.tuna.tsinghua.edu.cn/anaconda/cloud/conda-forge/
$HOME/miniconda3/bin/conda config --set show_channel_urls yes

# 更新 conda
$HOME/miniconda3/bin/conda update -n base conda -y

# 清理安装文件
# rm Miniconda3-latest-Linux-x86_64.sh

echo "Miniconda installation completed!"