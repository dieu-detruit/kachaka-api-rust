# kachaka-api

[Kachaka](https://kachaka.life/)用の非公式Rust APIクライアントライブラリです。

## 機能

公式では、PythonおよびROS 2向けのクライアントが用意されていますが、このリポジトリではRust向けのクライアントライブラリを提供します。

## インストール

`Cargo.toml`に以下を追加してください：

```toml
[dependencies]
kachaka-api = "0.1.0"
```

## 使用例

### 基本的な使用方法

```rust
use kachaka_api::{KachakaApiClient, StartCommandOptions};

#[tokio::main]
async fn main() {
    let mut client = KachakaApiClient::connect("http://kachaka-020.local:26400")
        .await
        .unwrap();
    let response = client
        .speak(
            "こんにちは、カチャカです",
            StartCommandOptions::default()
                .title("タイトル")
                .cancel_all(true),
        )
        .await
        .unwrap();
    println!("{:?}", response);
}

```

### その他の例

`examples`ディレクトリには以下のサンプルコードが含まれています：

- `watch_camera_image.rs`: カメラ画像のストリーミング
- `watch_compressed_camera_image.rs`: 圧縮されたカメラ画像のストリーミング
- `shelf_location_resolver.rs`: 棚と目的地の名前解決
- `simple_speak.rs`: コマンド実行のサンプル (発話とそれをキャンセルするサンプルになっています)
- `watch_update.rs`: ロボットの状態監視
- `watch_error.rs`: エラー監視
- `get_latest_info.rs`: 最新情報の取得

## ライセンス

[MIT License](LICENSE)

## 貢献

Issue、プルリクエストは大歓迎です。

## ⚠注意
- まだすべてのAPIがサポートされているわけではありません。
