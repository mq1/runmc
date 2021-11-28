import { BaseDirectory, appDir, join } from '@tauri-apps/api/path'
import { createDir, writeFile, removeDir, renameFile, readTextFile } from '@tauri-apps/api/fs'
import { open, Command } from '@tauri-apps/api/shell'
import { getName, getVersion } from '@tauri-apps/api/app'
import { downloadGameVersionMetaData } from './launchermeta'
import type { GameVersion } from './launchermeta'
import type { Account } from './accounts'

const configFile = 'config.yaml'

type Config = {
  gameVersionID: string
  type: string
}

const readInstanceConfig = async(name: string) => {
  const path = await join('instances', name, configFile)
  const contents = await readTextFile(path, { dir: BaseDirectory.App })
  const config = yaml.load(contents) as Config

  return config
}

export const newInstance = async(name: string, gameVersion: GameVersion) => {
  const dir = await join('instances', name)

  await createDir(dir, { dir: BaseDirectory.App })
  await downloadGameVersionMetaData(gameVersion)

  const config: Config = { gameVersionID: gameVersion.id, type: gameVersion.type }
  const contents = yaml.dump(config)
  const path = await join(dir, configFile)

  await writeFile({ contents, path }, { dir: BaseDirectory.App })
}

export const removeInstance = async(name: string) => {
  const dir = await join('instances', name)
  console.log(`Removing instance ${name}`)
  await removeDir(dir, { dir: BaseDirectory.App, recursive: true })
  console.log('Instance removed')
}

export const renameInstance = async(oldName: string, newName: string) => {
  const oldDir = await join('instances', oldName)
  const newDir = await join('instances', newName)

  await renameFile(oldDir, newDir, { dir: BaseDirectory.App })
}

export const openInstanceDir = async(name: string) => {
  const dir = await join(await appDir(), 'instances', name)
  await open(dir)
}

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
