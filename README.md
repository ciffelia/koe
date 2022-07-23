<h1 align="center">
  <img src="./icon/logo.png" alt="Koe" height="128">
</h1>

<p align="center">
  <a href="https://github.com/ciffelia/koe/actions?query=workflow%3ACI+branch%3Amain">
    <img src="https://github.com/ciffelia/koe/workflows/CI/badge.svg?branch=main" alt="CI status">
  </a>
  <a href="./LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-brightgreen.svg?style=flat" alt="MIT License">
  </a>
</p>

Koe は、指定されたテキストチャンネルに送信されたメッセージをボイスチャンネルで読み上げる Discord Bot です。
マイクをミュートにしている聞き専メンバーも会話に参加しやすくなります。

## 特徴

- [VOICEVOX ENGINE](https://github.com/VOICEVOX/voicevox_engine) を使った流暢な発音
- 日本語テキストチャットの読み上げに特化
- 特定の語句の読み方を設定する辞書機能を搭載
- Slash Commands に対応

## 使い方

Koe はテキストチャンネルで送信されたコマンドによって動作します。

### 読み上げ開始: `/join`, `/kjoin`

- VC に接続した状態で、読み上げたいテキストチャンネルで`/join`を送信すると、Bot が入室し読み上げを開始します。
- `/join`を送信したチャンネルの新規メッセージが読み上げられます。
- `/join`の代わりに`/kjoin`を使うこともできます。
  - サーバーに複数の Bot が存在していて、コマンドが重複しているときに便利です。

### 読み上げ終了: `/leave`, `/kleave`

- テキストチャンネルで`/leave`を送信すると、Bot が退室します。
  - どのチャンネルでも使えます。
  - VC に接続していないメンバーでも使えます。
- `/leave`の代わりに`/kleave`を使うこともできます。
  - サーバーに複数の Bot が存在していて、コマンドが重複しているときに便利です。
- 全員が VC から退室すると、Bot も自動的に退室します。

### 読み上げ中のメッセージをスキップ: `/skip`, `/kskip`

- `/skip`を送信すると、現在読み上げているメッセージの読み上げを中止して、次のメッセージを読み上げます。
- `/skip`の代わりに`/kskip`を使うこともできます。
  - サーバーに複数の Bot が存在していて、コマンドが重複しているときに便利です。

### 声を設定: `/voice`

- `/voice`を送信すると、あなたのメッセージを読み上げる際に使用する音源を設定するドロップダウンリストが表示されます。
- 設定はメンバーごとに保存されます。また、メンバーはサーバーごとに異なる音源を設定できます。
- はじめはメンバーごとにランダムな音源が割り当てられています。

### 辞書を閲覧・編集: `/dict`

- あらかじめ、特定の語句に別の読み方を設定しておくことができます。これを辞書機能といいます。
- 辞書はサーバーごとに設定できます。1 つのサーバーに 1 冊の辞書です。
- `/dict add 読み方を設定したい語句 読み方`を送信すると、辞書に語句を追加します。
- `/dict remove 語句`を送信すると、辞書から語句を削除します。
- `/dict view`を送信すると、辞書全体を表示します。

### 使い方を表示: `/help`

- このページの URL を表示します。

## 読み上げの仕組み

1. `/join`を送信したチャンネルでのメッセージを受信
2. スポイラー（ネタバレ、伏せ字）を削除
3. メッセージの送信者名と内容それぞれから URL を削除
4. 送信者名と内容を結合
   - ただし、同一メンバーによる 10 秒以内の連続したメッセージの場合は、名前は省略する
5. 辞書に登録されている語句を読み替え
6. 文字数が 60 文字を超えた場合、56 文字目以降は切り捨て、「以下略」を末尾に追加

## セットアップガイド

### 1. Discord Bot の登録

#### 1-1. アプリケーションの作成

1. [Discord Developer Portal](https://discord.com/developers/applications) を開き、新しくアプリケーションを作成します。
2. General Information の Client ID を控えておきます。
3. DescriptionにVOICEVOXや各音源のクレジット、使用上の注意事項などを入力します。ここで記入した内容はBotのプロフィールに表示されます。
4. 作成したアプリケーションで、Bot を有効にします。
5. Message Content Intent を有効にします。
6. Bot の Token を控えておきます。

#### 1-2. サーバーに Bot を追加

以下の URL にアクセスして、サーバーに Bot を追加します。`CLIENT_ID`は、先ほど控えた Client ID に置き換えてください。

```
https://discord.com/api/oauth2/authorize?client_id=CLIENT_ID&permissions=3146752&scope=bot%20applications.commands
```

##### 補足: Koe が使用する権限

###### OAuth2 Scopes

- `application.commands`
- `bot`

###### Bot Permissions

- General Permissions
  - View Channels
- Voice Permissions
  - Connect
  - Speak

### 2. Bot を起動

#### 2-1. 設定ファイルの構成

1. リポジトリを[ダウンロード](https://github.com/ciffelia/koe/archive/refs/heads/main.zip)し、適当な場所に展開します。以後、このディレクトリの中で作業を行います。
2. `config` ディレクトリと `docker-compose.yml` を残して、他のディレクトリやファイルは削除します。
3. `config` ディレクトリの `example.redis.conf` をテキストエディタで開いて Redis の設定を編集し、 `redis.conf` として保存します。
4. `config` ディレクトリの `example.voicevox_presets.yaml` をテキストエディタで開いて VOICEVOX のプリセットを編集し、 `voicevox_presets.yaml` として保存します。

#### 2-2. 環境変数の設定

`config` ディレクトリの `example.env` をテキストエディタで開いて、以下に示す環境変数を編集し、`.env` として保存します。

- `DISCORD_CLIENT_ID`（必須）: 1-1 で控えた Client ID を設定します。
- `DISCORD_BOT_TOKEN`（必須）: 1-1 で控えた Bot Token を設定します。
- `VOICEVOX_API_BASE`（必須）: VOICEVOX ENGINE の URL を設定します。Composeを使用する場合はデフォルトのままで問題ありません。
- `REDIS_URL`（必須）: Redis の URL を設定します。
  - 形式は `redis://[<username>][:<password>@]<hostname>[:port][/<db>]` です。
  - 詳細は https://docs.rs/redis#connection-parameters もご確認ください。
- `RUST_LOG`（任意）: `koe`に設定すると、詳細なログが出力されます。
- `SENTRY_DSN`（任意）: エラーをSentryに送信することができます。

#### 2-3. 起動

- `docker compose up --detach` でアプリケーションを起動します。
- `docker compose logs` でログを確認できます。
- `docker compose pull` でアプリケーションを更新します。ただし `docker-compose.yml` や設定ファイルが更新された場合は、手動でダウンロードする必要があります。
- `docker compose down` でアプリケーションを停止します。
- `docker compose down --volumes` でアプリケーションを停止し、Redis に保存されている設定をすべて削除します。
