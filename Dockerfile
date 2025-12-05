# ビルドステージ
FROM rust:1.91.1-slim AS builder

WORKDIR /app

# システム依存関係のインストール
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# ソースコードをコピー
COPY . .

# アプリケーションをビルド
RUN cargo build --release -p image-uploader-server

# ランタイムステージ
FROM debian:bookworm-slim

WORKDIR /app

# ランタイム依存関係のインストール
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# ビルドしたバイナリをコピー
COPY --from=builder /app/target/release/image-uploader-server /app/image-uploader-server

# .env ファイルをコピー（dotenvy で使用するため）
COPY .env /app/.env

# ポートを公開
EXPOSE 9090

# 環境変数は docker-compose で設定される

# アプリケーションを実行
CMD ["./image-uploader-server"]

