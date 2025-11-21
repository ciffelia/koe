# リリース手順

1. [`deployment/docker-compose.yml`](../deployment/docker-compose.yml)のイメージタグのバージョンを更新し、`main`ブランチにプッシュします。
2. `main`ブランチの最新コミットでCIが通っていることを確認します。
3. [Release workflow](https://github.com/ciffelia/koe/actions/workflows/release.yml)を開き、`Run workflow`をクリックしてワークフローを実行します。
4. 自動でGitHub ReleaseのDraftが作成され、Docker TagがContainer Registryにプッシュされます。
5. GitHub Releaseのリリースノートを編集して公開します。
