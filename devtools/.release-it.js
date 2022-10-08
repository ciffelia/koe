module.exports = {
  git: {
    commitMessage: 'chore: release v${version}'
  },
  npm: {
    publish: false
  },
  github: {
    release: true,
    releaseName: 'v${version}'
  },
  plugins: {
    '@release-it/conventional-changelog': {
      preset: 'conventionalcommits'
    }
  }
}
