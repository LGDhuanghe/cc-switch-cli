# 基础镜像使用 Ubuntu 18.04
FROM ubuntu:18.04

# 避免 apt 安装时的交互弹窗
ENV DEBIAN_FRONTEND=noninteractive

# 1. 安装基础构建工具及 Tauri 所需的系统依赖
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libwebkit2gtk-4.0-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# 2. 安装 Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# 3. 设置工作目录
WORKDIR /app

# 4. 复制当前目录下的所有文件到镜像中
# 注意：这会包含你本地的 src-tauri 文件夹
COPY . .

# 5. 进入 Tauri 目录执行编译
WORKDIR /app/src-tauri
RUN cargo build --release

# 编译出的二进制文件路径：/app/src-tauri/target/release/cc-switch-cli
