const semver = require('semver')

const main = () => {
  const { version } = parseArgs()

  const parsed = semver.parse(version)
  if (parsed === null) {
    console.error(`version ${version} is invalid`)
    process.exit(1)
  }

  const tags = generateTags(parsed)

  console.log(tags.join('\n'))
  console.log(`::set-output name=tags::${tags.join(' ')}`)
}

const parseArgs = () => {
  if (process.argv.length !== 3) {
    console.error('usage: node generateDockerTags.js <version>')
    console.error('example: node generateDockerTags.js 1.12.3-alpha.1')
    process.exit(1)
  }

  return {
    version: process.argv[2]
  }
}

const generateTags = (v) => {
  if (v.prerelease.length !== 0) {
    return [v.version]
  }

  if (v.major === 0) {
    return [
      `${v.major}.${v.minor}.${v.patch}`,
      `${v.major}.${v.minor}`,
      'latest'
    ]
  }

  return [
    `${v.major}.${v.minor}.${v.patch}`,
    `${v.major}.${v.minor}`,
    `${v.major}`,
    'latest'
  ]
}

main()
