import fs from 'fs'
import packageConfig from './package.json'

const version = packageConfig.version
const dateTime = new Date()
const signature = 'dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IDFEMDU4RTQ5Rjg4QzUwQzkKUldUSlVJejRTWTRGSFY1VVBhSGhYT2lZSXZQMndJYk81aWI1SHJVMGxMM0dBd0VEYThpai8zcTIK'

const json = {
  name: version,
  notes: `Release ${version}`,
  pub_date: dateTime.toISOString(),
  platforms: {
    darwin: {
      signature,
      url: `https://github.com/mq1/runmc/releases/download/${version}/runmc_${version}_x64.app.tar.gz`,
    },
    linux: {
      signature,
      url: `https://github.com/mq1/runmc/releases/download/${version}/runmc_${version}_amd64.AppImage.tar.gz`,
    },
    win64: {
      signature,
      url: `https://github.com/mq1/runmc/releases/download/${version}/runmc_${version}_x64.msi.zip`,
    },
  },
}

const text = JSON.stringify(json)

fs.writeFile('release.json', text, 'utf8', () => {})
