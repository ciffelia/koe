# FAQ

## GPUを使いたい

Koeが使用している音声合成エンジンであるVOICEVOX ENGINEでは、音声合成処理にNVIDIAのGPUを使用することができます。GPUを使用するには、`docker-compose.yml`を以下の手順で編集してください。

1. `voicevox/voicevox_engine:cpu`を`voicevox/voicevox_engine:nvidia`に変更します。
2. `voicevox`サービスに以下の行を追加します。

```yaml
    deploy:
      resources:
        reservations:
          devices:
            - capabilities: ["gpu"]
              runtime: nvidia
```

編集後の`voicevox`サービスの例を以下に示します。

```yaml
  voicevox:
    image: voicevox/voicevox_engine:nvidia-...
    restart: unless-stopped
    expose:
      # ...
    volumes:
      # ...
    healthcheck:
      # ...
    deploy:
      resources:
        reservations:
          devices:
            - capabilities: ["gpu"]
              runtime: nvidia
```
