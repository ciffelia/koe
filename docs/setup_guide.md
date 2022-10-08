# Koe セットアップガイド

この文章では Koe を起動するための手順を説明します。

## 0. システム要件

### 0-1. ハードウェア

読み上げ音声の合成は非常に負荷の大きい処理です。Koe を実行するコンピュータの性能が低い場合、テキストチャンネルにメッセージが送信されてからボイスチャンネルで読み上げられるまでの遅延が大きくなります。

Koe が使用している音声合成エンジンである VOICEVOX ENGINE では、音声合成処理に CPU または GPU を使用することができます。Bot を快適に使用するには高性能な CPU または GPU と 2GB 以上のメモリを搭載したマシンが必要です。

参考として、[@ciffelia](https://github.com/ciffelia) が使用しているマシンでの遅延は以下の通りです。

- CPU モード: Ryzen 5 5600X (メモリ 32GB): 1 秒程度
- GPU モード: RTX 3070 (メモリ 32GB): 1 秒程度
- CPU モード: Raspberry Pi 4 (メモリ 8GB): 15 秒程度

※起動直後はモデルの初期化処理が行われているため、遅延がより大きくなります。

### 0-2. ソフトウェア

Koe の実行には Docker および Docker Compose が必要です。あらかじめインストールしておいてください。なお、Koe が動作するには Redis と VOICEVOX ENGINE が必要ですが、これらは Docker Compose を用いて起動するため事前のインストールは不要です。

## 1. Discord Bot の登録

### 1-1. アプリケーションの作成

1. [Discord Developer Portal](https://discord.com/developers/applications) を開き、新しくアプリケーションを作成します。
2. General Information ページに記載されている Application ID (Client ID) を控えておきます。
3. Description に VOICEVOX や各音源のクレジット、使用上の注意事項などを入力します。ここで記入した内容は Bot のプロフィールに表示されます。
4. Bot ページに移動し、Add Bot をクリックして Bot を有効にします。
5. Message Content Intent を有効にします。
6. Reset Token をクリックして Token を生成し、控えておきます。

### 1-2. サーバーに Bot を追加

以下の URL にアクセスしてサーバーに Bot を追加します。URL の`CLIENT_ID`は先ほど控えた Application ID に置き換えてください。

```
https://discord.com/api/oauth2/authorize?client_id=CLIENT_ID&permissions=3146752&scope=bot%20applications.commands
```

<details>
  <summary>参考: Koe が使用する権限</summary>
  
  - OAuth2 Scopes
    - `application.commands`
    - `bot`
  - Bot Permissions
    - General Permissions
      - View Channels
    - Voice Permissions
      - Connect
      - Speak
</details>

## 2. 設定ファイルの準備

### 2-1. サンプル設定ファイルのダウンロード

1. リポジトリを[ダウンロード](https://github.com/ciffelia/koe/archive/refs/heads/main.zip)し、適当な場所に展開します。
2. `deployment` ディレクトリを残し、他のファイルやディレクトリは削除します。以後、このディレクトリの中で作業を行います。

### 2-2. Redis のパスワード設定

1. `config` ディレクトリにある `redis.conf`をテキストエディタで開きます。
2. `YOUR_STRONG_PASSWORD` を適当なパスワードに変更します。

### 2-3. VOICEVOX ENGINE のプリセット設定（任意）

1. `config` ディレクトリにある `voicevox_presets.yaml`をテキストエディタで開きます。
2. 必要に応じてプリセットを変更します。

### 2-4. Koe の設定

1. `config` ディレクトリにある `koe.yaml`をテキストエディタで開きます。
2. 次の設定を編集します。
   - `discord.client_id`: 1-1 で控えた Client ID を設定します。
   - `discord.bot_token`: 1-1 で控えた Bot Token を設定します。
   - `voicevox.api_base`: VOICEVOX ENGINE の URL を設定します。
     - Docker Compose を使用する場合はデフォルトのままで問題ありません。
   - `redis.url`: Redis に接続するための URL を設定します。
     - 形式は `redis://[<username>][:<password>@]<hostname>[:port][/<db>]` です。
     - Docker Compose を使用する場合は`YOUR_STRONG_PASSWORD`を Redis のパスワードに置き換えるのみで問題ありません。
     - 詳細は https://docs.rs/redis#connection-parameters をご確認ください。

### 2-5. 環境変数の設定（任意）

`docker-compose.yml` から以下の環境変数を設定することができます。どちらも原則として設定する必要はありませんが、デバッグ時に役立ちます。

- `KOE_CONFIG`: 設定ファイルの場所を設定します。
  - デフォルトでは `/etc/koe.yaml` となっています。
- `RUST_LOG`: ログレベルを設定します。
  - `koe`に設定すると詳細なログが出力されます。
  - 詳細は https://docs.rs/env_logger#enabling-logging をご確認ください。
- `SENTRY_DSN`: Sentry の DSN を設定します。
  - 設定するとエラーを Sentry に送信することができます。

## 3. 起動

Docker Compose を使用する場合は以下のコマンドで開始・停止等の操作を行うことができます。詳細は https://docs.docker.com/compose/ をご確認ください。

- `docker compose up --detach`
  - Koe, Redis, VOICEVOX ENGINE を起動します。
- `docker compose logs`
  - ログを確認します。
- `docker compose down`
  - Koe, Redis, VOICEVOX ENGINE を停止します。
- `docker compose down --volumes`
  - Koe, Redis, VOICEVOX ENGINE を停止し、Redis に保存されている設定をすべて削除します。
- `docker compose pull`
  - コンテナイメージを更新します。

## 4. 更新

1. `docker compose down`でアプリケーションを停止します。
2. [`docker-compose.yml`](https://github.com/ciffelia/koe/blob/main/docker-compose.yml) をダウンロードし上書きします。
3. `docker compose pull`でコンテナイメージを更新します。
4. `docker compose up --detach` でアプリケーションを起動します。

---

不明な点がありましたら[Discussions](https://github.com/ciffelia/koe/discussions)でご相談ください。
