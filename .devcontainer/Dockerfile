# ベースイメージ
FROM ubuntu:22.04

# 非対話モードでのapt実行設定
ENV DEBIAN_FRONTEND=noninteractive

# 基本パッケージとRust用依存パッケージの導入
RUN apt-get update && apt-get install -y \
    curl \
    git \
    unzip \
    gzip \
    xz-utils \
    ca-certificates \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# protoのインストール
RUN bash -c "bash <(curl -fsSL https://moonrepo.dev/install/proto.sh) --yes"

# protoのPATHを通す
ENV PATH="/root/.proto/bin:${PATH}"

# デフォルトの作業ディレクトリ
WORKDIR /workspace
