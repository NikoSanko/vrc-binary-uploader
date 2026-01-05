# VRC Binary Uploader API
## 概要
VRChat のワールドギミック向けに画像などを独自形式に変換してアップロードするAPIサーバーです。

## 開発環境の準備
### 必要なツール
- [go-task](https://taskfile.dev/) - タスクランナー
- Docker - コンテナ実行環境
- Rust 1.91以上 - 開発言語

### セットアップ手順
1. **go-task のインストール**
    [公式Doc参照](https://taskfile.dev/docs/installation)

2. **開発環境のチェック**
   ```bash
   task check
   ```
   このコマンドで以下が確認されます：
   - Docker がインストールされているか
   - Rust 1.91以上がインストールされているか

## APIサーバーの起動
### ローカル開発環境での起動
```bash
task dev
```
サーバーがフォアグランドで起動します。

### Dockerコンテナでの起動
```bash
# コンテナを起動
task start

# コンテナを停止
task stop
```

サーバーは `http://localhost:9090` でアクセス可能です。（コンテナのポートは環境変数に影響されるので要修正）

## APIの追加や仕様変更
APIの仕様は OpenAPI 形式で `docs/api.yaml` に定義されています。

### 手順
1. **API仕様の更新**
   `docs/api.yaml` を編集して、エンドポイントやスキーマを追加・変更します。

2. **コードの自動生成**
   ```bash
   task gen:api
   ```

   このコマンドで OpenAPI 仕様から Rust のコードが自動生成されます。

3. **実装の追加**
   生成されたコードをベースに、ハンドラーやビジネスロジックを実装します。

## その他のタスク
- `task test` - テストを実行
- `task gen:crunch` - pngやjpeg画像を dds 形式に変換する crunch バイナリをビルド
