module.exports = {
  git: {
    commitMessage: 'chore: release v${version}'
  },
  npm: {
    publish: false
  },
  github: {
    release: true,
    releaseName: 'v${version}',
    assets: ['./koe_*.zip']
  },
  plugins: {
    '@release-it/conventional-changelog': {
      preset: 'conventionalcommits'
    }
  },
  hooks: {
    'after:bump':
      "sed -i 's/${latestVersion}/${version}/g' ../deployment/docker-compose.yml",
    'before:github:release':
      "cp -r ../deployment ./koe && zip -r 'koe_${version}.zip' ./koe && rm -rf ./koe"
  }
}
