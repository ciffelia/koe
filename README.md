# Koe

[![MIT License](https://img.shields.io/badge/license-MIT-brightgreen.svg?style=flat)](LICENSE)

Discord 読み上げ Bot

## 特徴

- Google Text-to-Speech API を使った流暢な発音
- 日本語テキストチャットの読み上げに特化
- Slash Commands に対応

## 使い方

### 読み上げ開始: `/join`

- VC に接続した状態で、読み上げたいテキストチャンネルで`/join`を送信すると、Bot が入室し読み上げを開始します。
  - `/join`を送信したチャンネルの新規メッセージが読み上げられます。

### 読み上げ終了: `/leave`

- テキストチャンネルで`/leave`を送信すると、Bot が退室します。
  - どのチャンネルでも使えます。
  - VC に接続していないメンバーでも使えます。
- 全員が VC から退室すると、Bot も自動的に退室します。

## セットアップガイド

### 1. Google Text-to-Speech API の登録

#### 1-1. プロジェクトの作成

[Google Cloud Platform Console](https://console.cloud.google.com/) を開き、新しくプロジェクトを作成します。

#### 1-2. Text-to-Speech API のセットアップ

1. [公式ガイド](https://cloud.google.com/text-to-speech/docs/before-you-begin) にしたがって Text-to-Speech API を有効化し、JSON キーをダウンロードします。
2. ダウンロードした JSON キーをファイルシステム上の安全な場所に配置し、ファイルパスを控えておきます。

### 2. Discord Bot の登録

#### 2-1. アプリケーションの作成

1. [Discord Developer Portal](https://discord.com/developers/applications) を開き、新しくアプリケーションを作成します。
2. `General Information`の`Client ID`を控えておきます。
3. 作成したアプリケーションで、Bot を有効にします。
4. `Bot`の`Token`を控えておきます。

#### 2-2. サーバーに Bot を追加

以下の URL にアクセスして、サーバーに Bot を追加します。先ほど控えた`Client ID`を使います。

```
https://discord.com/api/oauth2/authorize?client_id=CLIENT_ID&permissions=3146752&scope=bot%20applications.commands
```

##### 補足: Bot に必要な権限

###### OAuth2 Scopes

- application.commands
- bot

###### Bot Permissions

- General Permissions
  - View Channels
- Voice Permissions
  - Connect
  - Speak

### 3. Bot を起動

#### 3-1. 環境変数の設定

以下の環境変数をそれぞれ設定します。

- `GOOGLE_APPLICATION_CREDENTIALS`（必須）: JSON キーのファイルパスを設定します。絶対パス・相対パスどちらも使えます。
- `DISCORD_CLIENT_ID`（必須）: 2-1 で控えた Client ID を設定します。
- `DISCORD_BOT_TOKEN`（必須）: 2-1 で控えた Bot Token を設定します。
- `RUST_LOG`（任意）: `koe`に設定すると、詳細なログが出力されます。

#### 3-2. 起動

Bot を起動します。
