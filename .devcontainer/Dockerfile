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
RUN curl -fsSL -o proto.sh https://moonrepo.dev/install/proto.sh && \
    # ここで proto.sh のチェックサムを検証することを推奨します。 (例: echo "EXPECTED_CHECKSUM  proto.sh" | sha256sum -c -) && \
    bash proto.sh --yes && \
    rm proto.sh

# protoのPATHを通す
ENV PATH="/root/.proto/bin:${PATH}"

# デフォルトの作業ディレクトリ
WORKDIR /workspace
