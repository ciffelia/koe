module.exports = {
  git: {
    commitMessage: 'chore: release v${version}'
  },
  npm: {
    publish: false
  },
  github: {
    release: true
  },
  plugins: {
    '@release-it/conventional-changelog': {
      preset: 'conventionalcommits'
    }
  }
}
