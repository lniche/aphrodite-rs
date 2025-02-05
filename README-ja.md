# Aphrodite Axum API Scaffold

[English](README.md) | [简体中文](README-zh.md) | [日本語](README-ja)

Aphrodite は、[api-tpl-rs](https://github.com/shenghui0779/yiirs)
をベースに開発されたテンプレートプロジェクトで、開発者がフレームワークの使用フローを迅速に理解し、深く学ぶことを目的としています。このプロジェクトは、一般的な開発シーンを網羅した包括的なサンプルコードと設定を提供し、学習と実践をサポートします。さらに、Aphrodite
にはコンテナーデプロイメントテンプレートも含まれており、現代のクラウド環境でプロジェクトを簡単にデプロイおよび管理できるようになっており、開発者がアプリケーションを効率的に構築および公開する手助けをします。

## 技術スタック

| 技術                                                              | 説明                                        |
|-----------------------------------------------------------------|-------------------------------------------|
| [tokio](https://github.com/tokio-rs/tokio)                      | 非同期ランタイム、さまざまな非同期機能とフル機能をサポート             |
| [clap](https://github.com/clap-rs/clap)                         | コマンドライン引数解析ライブラリ、派生マクロをサポート               |
| [thiserror](https://github.com/dtolnay/thiserror)               | エラーハンドリングライブラリ、簡潔なエラー定義を提供                |
| [anyhow](https://github.com/dtolnay/anyhow)                     | 柔軟なエラーハンドリングライブラリ、エラー伝播を簡素化               |
| [base64](https://crates.io/crates/base64)                       | Base64 エンコードとデコードライブラリ                    |
| [time](https://crates.io/crates/time)                           | 時間処理ライブラリ、マクロ、ローカルタイムゾーン、フォーマットと解析機能をサポート |
| [serde](https://serde.rs/)                                      | データシリアル化とデシリアル化ライブラリ、派生マクロをサポート           |
| [serde_json](https://crates.io/crates/serde_json)               | JSON データのシリアル化とデシリアル化ライブラリ                |
| [tracing](https://github.com/tokio-rs/tracing)                  | 非同期アプリケーションのロギングフレームワーク                   |
| [tracing-subscriber](https://github.com/tokio-rs/tracing)       | ログサブスクライバー、JSON フォーマットをサポート               |
| [tracing-appender](https://crates.io/crates/tracing-appender)   | ログアペンダー、ログをファイルに書き込む機能                    |
| [hyper](https://github.com/hyperium/hyper)                      | 高性能な HTTP 実装                              |
| [http](https://crates.io/crates/http)                           | HTTP メッセージおよびリクエスト/レスポンス処理ライブラリ           |
| [http-body](https://crates.io/crates/http-body)                 | HTTP リクエストとレスポンスボディ処理ライブラリ                |
| [http-body-util](https://crates.io/crates/http-body-util)       | HTTP リクエスト体およびレスポンス体のユーティリティ              |
| [tower-http](https://github.com/tower-rs/tower-http)            | Tower ベースの HTTP 関連ミドルウェア                  |
| [axum](https://github.com/tokio-rs/axum)                        | Tokio ベースの非同期 Web フレームワーク                 |
| [axum-extra](https://crates.io/crates/axum-extra)               | Axum の追加機能と拡張ライブラリ                        |
| [nanoid](https://crates.io/crates/nanoid)                       | 一意な ID を生成するライブラリ                         |
| [validator](https://github.com/Keats/validator)                 | データ検証ライブラリ、派生マクロをサポート                     |
| [sea-orm](https://github.com/SeaQL/sea-orm)                     | ORM フレームワーク、MySQL や他のデータベースをサポート          |
| [redis](https://github.com/redis/redis-rs)                      | Redis クライアントライブラリ、クラスタと R2D2 接続プールをサポート   |
| [utoipa](https://github.com/utahta/utoipa)                      | API ドキュメント生成ツール                           |
| [utoipa-swagger-ui](https://crates.io/crates/utoipa-swagger-ui) | Utoipa ベースの Swagger UI 統合                 |
| [rand](https://crates.io/crates/rand)                           | ランダム数生成ライブラリ                              |
| [const-hex](https://crates.io/crates/const-hex)                 | 定数の16進数文字列変換ライブラリ                         |
| [openssl](https://crates.io/crates/openssl)                     | OpenSSL バインディング、暗号化と復号化をサポート              |
| [digest](https://crates.io/crates/digest)                       | メッセージダイジェストアルゴリズムライブラリ                    |
| [md-5](https://crates.io/crates/md5)                            | MD5 ハッシュライブラリ                             |
| [sha1](https://crates.io/crates/sha1)                           | SHA-1 ハッシュライブラリ                           |
| [sha2](https://crates.io/crates/sha2)                           | SHA-2 ハッシュライブラリ                           |
| [hmac](https://crates.io/crates/hmac)                           | HMAC（ハッシュメッセージ認証コード）ライブラリ                 |
| [config](https://github.com/mehcode/config-rs)                  | 設定管理ライブラリ、複数のフォーマットをサポート                  |
| [chrono](https://crates.io/crates/chrono)                       | 日付と時間のライブラリ                               |
| [mobc](https://crates.io/crates/mobc)                           | 接続プールライブラリ、非同期操作をサポート                     |
| [r2d2](https://crates.io/crates/r2d2)                           | 接続プールライブラリ、同期操作をサポート                      |

## 特徴

- **ユーザー認証と認可**：基本的なユーザーログインと権限付与機能を提供します。
- **分散ロック**：Redisをベースにした分散ロックを実装し、分散環境下でのリソースの安全性を確保します。
- **ミドルウェアサポート**：認証、リクエストログ、CORS処理など、よく使われるミドルウェアを内蔵しています。
- **統一された出力フォーマット**：簡単に使えるAPI Resultの統一出力方式を提供し、APIのレスポンスフォーマットを標準化して、インターフェースの一貫性を向上させます。
- **APIモジュール設計**：モジュール化されたAPI設計をサポートし、拡張と保守が容易です。
- **Swaggerドキュメント統合**：APIドキュメントを自動生成し、フロントエンド開発とテストを容易にします。

## ディレクトリ構造

```
.
├── scripts/              # 実行可能なスクリプト
├── database/             # データベース関連
├── deploy/               # デプロイ関連ファイル
├── docs/                 # プロジェクトドキュメント
├── src/                  # コアディレクトリ
│   ├── app/              # アプリケーションのコアコード
│   ├── pkg/              # 共通モジュール
├── storage/              # ファイルストレージ
├── tests/                # テストファイル
└── README.md             # プロジェクト説明
```

## ローカルでの実行

```bash
# 1. プロジェクトのコードリポジトリをクローン
git clone https://github.com/lniche/aphrodite-rs.git
cd aphrodite-rs

# 2. 設定ファイルの準備
mv config.toml.example config.toml

# 3. 依存関係の処理
# Rust と Cargo 環境がインストールされていることを確認してください
cargo build

# 4. データベースの初期化
database/migrations/V1.0.0__initial_schema.sql

# 5. サービスの起動
cargo run -- serve
```

## リポジトリの活動

![Alt](https://repobeats.axiom.co/api/embed/7d3f9b2c6f3ee0be57460b614334ff2739f36b92.svg "Repobeats analytics image")

## ライセンス

このプロジェクトは MIT ライセンスに従っています。

## 感謝の意

すべてのコントリビューターとサポーターに感謝します。皆さんのご支援が私たちにとって非常に重要です！
