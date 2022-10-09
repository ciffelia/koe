const semver = require('semver');
const pkg = require('../package.json');

const main = () => {
  const { version } = pkg;

  const parsed = semver.parse(version);
  if (parsed === null) {
    console.error(`version ${version} is invalid`);
    process.exit(1);
  }

  console.log(generateTags(parsed).join(' '));
};

const generateTags = (v) => {
  if (v.prerelease.length !== 0) {
    return [v.version];
  }

  if (v.major === 0) {
    return [
      `${v.major}.${v.minor}.${v.patch}`,
      `${v.major}.${v.minor}`,
      'latest',
    ];
  }

  return [
    `${v.major}.${v.minor}.${v.patch}`,
    `${v.major}.${v.minor}`,
    `${v.major}`,
    'latest',
  ];
};

main();
