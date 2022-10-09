const path = require('path');
const fs = require('fs');

const changelogHeaderTemplate = fs.readFileSync(
  path.join(__dirname, './template/changelog/header.hbs'),
  'utf-8',
);

module.exports = {
  git: {
    commitMessage: 'chore: release v${version}',
  },
  npm: {
    publish: false,
  },
  github: {
    release: true,
    releaseName: 'v${version}',
    assets: ['./koe_*.zip'],
  },
  plugins: {
    '@release-it/conventional-changelog': {
      preset: 'conventionalcommits',
      writerOpts: {
        headerPartial: changelogHeaderTemplate,
      },
    },
  },
  hooks: {
    'after:bump':
      "sed -i 's/koe:${latestVersion}/koe:${version}/g' ../deployment/docker-compose.yml",
    'before:git:release': 'git add ../deployment/docker-compose.yml',
    'before:github:release':
      "cp -r ../deployment ./koe && zip -r 'koe_${version}.zip' ./koe && rm -rf ./koe",
  },
};
