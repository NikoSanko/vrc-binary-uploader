# ビルドステージ
FROM rust:1.91.1-slim AS builder

WORKDIR /app

# システム依存関係のインストール
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    cmake \
    build-essential \
    git \
    && rm -rf /var/lib/apt/lists/*

# ソースコードをコピー
COPY . .

# crunch リポジトリを指定のコミットからクローンしてビルド
RUN git clone https://github.com/DaemonEngine/crunch.git crunch_repo \
    && cd crunch_repo \
    && git checkout e242bb2a7ae7efc44d144fd5d621b35baddeab25 \
    && cmake -S . -B build \
    && cmake --build build --parallel $(nproc) \
    && mkdir -p /app/server/resources/bin \
    && cp build/crunch /app/server/resources/bin/crunch

# アプリケーションをビルド
RUN cargo build --release -p image-uploader-server

# ランタイムステージ
FROM debian:trixie-slim

WORKDIR /app

# ランタイム依存関係のインストール
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# ビルドしたバイナリをコピー
COPY --from=builder /app/target/release/image-uploader-server /app/image-uploader-server
COPY --from=builder /app/server/resources /app/server/resources

# .env ファイルをコピー（dotenvy で使用するため）
COPY .env /app/.env

# ポートを公開
EXPOSE 9090

# アプリケーションを実行
CMD ["./image-uploader-server"]

