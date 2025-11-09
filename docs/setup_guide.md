# Koeセットアップガイド

この文章ではKoeを起動するための手順を説明します。

## 0. システム要件

### 0-1. ハードウェア

読み上げ音声の合成は非常に負荷の大きい処理です。Koeを実行するコンピュータの性能が低い場合、テキストチャンネルにメッセージが送信されてからボイスチャンネルで読み上げられるまでの遅延が大きくなります。

Koeが使用している音声合成エンジンであるVOICEVOX ENGINEでは、音声合成処理にCPUまたはGPUを使用することができます。Botを快適に使用するには高性能なCPUまたはGPUと2GB以上のメモリを搭載したマシンが必要です。

参考までに、[@ciffelia](https://github.com/ciffelia)が使用しているマシンでの遅延は以下の通りです。

- CPU: Ryzen 5 5600X: 1秒程度
- CPU: Raspberry Pi 4 (8GB): 15秒程度
- GPU: RTX 3070: 1秒程度

※起動直後はモデルの初期化処理が行われているため、遅延がより大きくなります。

### 0-2. ソフトウェア

Koeの実行にはDockerおよびDocker Composeが必要です。あらかじめインストールしておいてください。なお、Koeが動作するにはRedisとVOICEVOX ENGINEが必要ですが、これらはDocker Composeを用いて起動するため事前のインストールは不要です。

## 1. Discord Botの登録

### 1-1. アプリケーションの作成

1. [Discord Developer Portal](https://discord.com/developers/applications)を開き、新しくアプリケーションを作成します。
2. General Informationページに記載されているApplication ID (Client ID)を控えておきます。
3. DescriptionにVOICEVOXや各音源のクレジット、使用上の注意事項などを入力します。ここで記入した内容はBotのプロフィールに表示されます。
4. Installationページに移動し、Install LinkをNoneに設定します。
5. Botページに移動します。
6. Reset TokenをクリックしてTokenを生成し、控えておきます。
7. Public Botを無効にします。
8. Message Content Intentを有効にします。
9. 画面下部のSave Changesをクリックして設定を保存します。

<details>
<summary>参考: プロフィールの記入例</summary>

> KoeはVOICEVOX及び以下に記す音源を用いて音声合成を行っています。公序良俗に反する内容の読み上げなど、VOICEVOXや各音源の利用規約に違反する行為はお控えください。  
> 四国めたん, ずんだもん, 春日部つむぎ, 雨晴はう, 波音リツ, 玄野武宏, 白上虎太郎, 青山龍星, 冥鳴ひまり, 九州そら, もち子（CV: 明日葉よもぎ）, 剣崎雌雄

</details>

### 1-2. サーバーにBotを追加

以下のURLにアクセスしてサーバーにBotを追加します。URLの`CLIENT_ID`は先ほど控えたApplication IDに置き換えてください。

```
https://discord.com/api/oauth2/authorize?client_id=CLIENT_ID&permissions=3146752&scope=bot%20applications.commands
```

Botを他のサーバーにも追加したい場合は、このURLに再度アクセスしてください。

<details>
  <summary>参考: Koeが使用する権限</summary>
  
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

### 2-1. 設定ファイルのダウンロード

1. [最新のリリース](https://github.com/ciffelia/koe/releases/latest)を開き、`koe_x.x.x.zip`をダウンロードします。
2. ダウンロードしたアーカイブを展開します。以後、このディレクトリの中で作業を行います。

### 2-2. Redisのパスワード設定

1. `config/redis.conf`をテキストエディタで開きます。
2. `YOUR_STRONG_PASSWORD`を適当なパスワードに変更します。

### 2-3. VOICEVOX ENGINEのプリセット設定（任意）

1. `config/voicevox_presets.yaml`をテキストエディタで開きます。
2. 必要に応じてプリセットを変更します。

### 2-4. Koeの設定

1. `config/koe.yaml`をテキストエディタで開きます。
2. 次の設定を書き換えます。
   - `discord.client_id`: 1-1で控えたClient ID
   - `discord.bot_token`: 1-1で控えたBot Token
   - `voicevox.api_base`: VOICEVOX ENGINEのURL
     - Docker Composeを使用する場合はデフォルトのままで問題ありません。
   - `redis.url`: Redisに接続するためのURL
     - 形式は`redis://[<username>][:<password>@]<hostname>[:port][/<db>]`です。
     - Docker Composeを使用する場合は`YOUR_STRONG_PASSWORD`をRedisのパスワードに置き換えるのみで問題ありません。
     - 詳細は https://docs.rs/redis#connection-parameters をご確認ください。

### 2-5. 環境変数の設定（任意）

`docker-compose.yml`から下記の環境変数を設定することができます。いずれも原則として設定する必要はありませんが、デバッグ時に役立ちます。

- `KOE_CONFIG`: 設定ファイルの場所
  - デフォルトでは`/etc/koe.yaml`となっています。
- `RUST_LOG`: ログレベル
  - `koe`に設定すると詳細なログが出力されます。
  - 詳細は https://docs.rs/env_logger#enabling-logging をご確認ください。

## 3. 起動

下記のコマンドで開始・停止等の操作を行うことができます。詳細は https://docs.docker.com/compose/ をご確認ください。

- `docker compose up --detach`
  - Koe, Redis, VOICEVOX ENGINEを起動します。
- `docker compose logs`
  - ログを確認します。
- `docker compose down`
  - Koe, Redis, VOICEVOX ENGINEを停止します。
- `docker compose down --volumes`
  - Koe, Redis, VOICEVOX ENGINEを停止し、Redisに保存されている設定をすべて削除します。
- `docker compose pull`
  - コンテナイメージを更新します。

---

不明な点がありましたら[Discussions](https://github.com/ciffelia/koe/discussions)でご相談ください。
