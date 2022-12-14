# リリース手順

1. `main`ブランチの最新コミットで CI が通っていることを確認します。
2. [Create release](https://github.com/ciffelia/koe/actions/workflows/release.yml) を開き、`Run workflow` をクリックしてワークフローを実行します。
3. 自動で GitHub Release が作成され、Docker Tag が Container Registry にプッシュされます。
4. GitHub Release のリリースノートを編集します。
