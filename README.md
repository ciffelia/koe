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

## インストール方法

[セットアップガイド](docs/setup_guide.md)をご覧ください。

## 不具合の報告

[Issues](https://github.com/ciffelia/koe/issues) から日本語または英語で報告をお願い致します。

## 質問・相談

[Discussions](https://github.com/ciffelia/koe/discussions) から日本語または英語でご相談ください。
