import { BaseDirectory, appDir } from '@tauri-apps/api/path'
import { createDir, writeFile, readDir, removeDir, renameFile, readTextFile } from '@tauri-apps/api/fs'
import { open, Command } from '@tauri-apps/api/shell'
import { getName, getVersion } from '@tauri-apps/api/app'
import * as yaml from 'js-yaml'
import { downloadGameVersionMetaData } from './launchermeta'
import { read as readConfig } from './config'
import type { GameVersion } from './launchermeta'
import type { Account } from './accounts'

const configFile = 'config.yaml'

type Config = {
  gameVersionID: string
  type: string
}

const readInstanceConfig = (name: string) =>
  readTextFile(`instances/${name}/${configFile}`, { dir: BaseDirectory.App })
    .then(text => yaml.load(text) as Config)

export const newInstance = async(name: string, gameVersion: GameVersion) => {
  const dir = `instances/${name}`

  await createDir(dir, { dir: BaseDirectory.App, recursive: true })
  await downloadGameVersionMetaData(gameVersion)

  const config: Config = { gameVersionID: gameVersion.id, type: gameVersion.type }
  const text = yaml.dump(config)
  await writeFile({ contents: text, path: `${dir}/${configFile}` }, { dir: BaseDirectory.App })
}

export const listInstances = () =>
  readDir('instances', { dir: BaseDirectory.App })
    .then(list => list.filter(file => file.name !== undefined))
    .then(list => list.map(file => file.name!))

export const removeInstance = async(name: string) => {
  console.log(`Removing instance ${name}`)
  await removeDir(`instances/${name}`, { dir: BaseDirectory.App, recursive: true })
  console.log('Instance removed')
}

export const renameInstance = (oldName: string, newName: string) =>
  renameFile(`instances/${oldName}`, `instances/${newName}`, { dir: BaseDirectory.App })

export const openInstanceDir = (name: string) =>
  appDir()
    .then(baseDir => open(`${baseDir}/instances/${name}`))

// INCOMPLETE
export const runInstance = async(name: string, account: Account) => {
  const baseDir = await appDir()
  const dir = `${baseDir}/instances/${name}`

  const globalConfig = await readConfig()
  const instanceConfig = await readInstanceConfig(name)
  const launcherName = await getName()
  const launcherVersion = await getVersion()

  const command = new Command(globalConfig.java.path, [
    '--username', account.name,
    '--version', instanceConfig.gameVersionID,
    '--gameDir', '.',
    '--assetsDir', '../../assets',
    '--assetIndex', `../../assets/indexes/${instanceConfig.gameVersionID}`,
    '--uuid', account.id,
    '--accessToken', account.accessToken,
    '--userType', 'mojang',
    '--versionType', instanceConfig.type,

    `-Dminecraft.launcher.brand=${launcherName}`,
    `-Dminecraft.launcher.version=${launcherVersion}`,
    `-Xmx${globalConfig.java.memory}`,
    `-Xms${globalConfig.java.memory}`,
  ], { cwd: dir })

  command.on('close', data => console.log(`command finished with code ${data.code} and signal ${data.signal}`))
  command.on('error', error => console.error(`command error: "${error}"`))
  command.stdout.on('data', line => console.log(`command stdout: "${line}"`))
  command.stderr.on('data', line => console.log(`command stderr: "${line}"`))

  await command.spawn()
}
