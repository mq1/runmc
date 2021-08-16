import { readTextFile, writeFile, BaseDirectory } from '@tauri-apps/api/fs'
import * as yaml from 'js-yaml'

const configFile = 'config.yaml'

export type Config = {
  locale: string
  java: {
    path: string
    memory: string
  }
}

export const getDefaultConfig = (): Config => ({
  locale: 'en',
  java: {
    path: 'java',
    memory: '2G',
  },
})

const newConfig = async() => {
  const config = getDefaultConfig()
  const text = yaml.dump(config)
  await writeFile({ contents: text, path: configFile }, { dir: BaseDirectory.App })

  return config
}

export const read = () =>
  readTextFile(configFile, { dir: BaseDirectory.App })
    .then(text => yaml.load(text) as Config)
    .catch(() => newConfig())

export const write = async(config: Config) => {
  const text = yaml.dump(config)
  await writeFile({ contents: text, path: configFile }, { dir: BaseDirectory.App })
}
