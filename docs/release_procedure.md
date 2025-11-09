# リリース手順

1. `main`ブランチの最新コミットでCIが通っていることを確認します。
2. [Create release](https://github.com/ciffelia/koe/actions/workflows/release.yml)を開き、`Run workflow`をクリックしてワークフローを実行します。
3. 自動でGitHub Releaseが作成され、Docker TagがContainer Registryにプッシュされます。
4. GitHub Releaseのリリースノートを編集します。
