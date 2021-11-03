import { readTextFile, writeFile, BaseDirectory } from '@tauri-apps/api/fs'
import * as yaml from 'js-yaml'

const path = 'config.yaml'

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
  const contents = yaml.dump(config)
  await writeFile({ contents, path }, { dir: BaseDirectory.App })

  return config
}

export const read = () =>
  readTextFile(path, { dir: BaseDirectory.App })
    .then(contents => yaml.load(contents) as Config)
    .catch(() => newConfig())

export const write = async(config: Config) => {
  const contents = yaml.dump(config)
  await writeFile({ contents, path }, { dir: BaseDirectory.App })
}
