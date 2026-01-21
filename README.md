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
# 画像をDDSに変換するための実行ファイルをビルド
task gen:crunch

# モックストレージが必要なら下記で起動
task mock-storage 

# フォアグラウンドでサーバー起動
task dev
```
モックストレージが起動しているなら、`http://localhost:{MOCK_STORAGE_PORT}/upload/{アップロード先としたいファイル名}` を presignedUrl として使えます

### Dockerコンテナでの起動
```bash
# コンテナを起動
task start

# コンテナを停止
task stop
```

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
- `task test` - テストを実行（crunchを使用して実際に画像のDDS変換テストも行うので、事前に `task gen:crunch` が必要）
- `task gen:crunch` - pngやjpeg画像を dds 形式に変換する crunch バイナリをビルド

## 画像変換の仕様
複数画像をそれぞれDDSに変換したのち、独自のファイルフォーマットとして1ファイルにシリアライズする

### ファイル構造
- 4 byte
  - 画像枚数を表す
  - Int32
  - リトルエンディアン
- 4 byte × 画像枚数
  - 各DDSデータのサイズを表す
  - リトルエンディアン
- 不定 byte
  - シリアライズされたDDSデータ

### 制約
- アップロードする画像はjpeg形式
  - png でも jpeg でも DDS に変換した際の画像品質に差はあまりなく、ファイルサイズは変換前の形式によらない
- 画像の縦横のピクセル数は 4 の倍数
  - GPUが画像を解釈する際に 4bit ずつ処理することに起因
- シリアライズして出力されたファイルのサイズは 10 MB以下
  - VRChat の StringLoading が 10 MBまでしか取得できない制約に起因

### 参考
DDS変換後のファイルサイズ
- 724  × 1024 ＝＞ 484 KB
- 1448 × 2048 ＝＞ 1932 KB
