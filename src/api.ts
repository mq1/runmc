import { readDir, Dir, createDir, removeDir } from 'tauri/api/fs'
import { promisified } from 'tauri/api/tauri'

export interface Version {
  id: string,
  type?: string,
  url: string
}

export const getAvailableVersions = async (): Promise<Version[]> => {
  const r = await fetch('https://launchermeta.mojang.com/mc/game/version_manifest.json')
  const j = await r.json()

  return j.versions
}

export const getInstalledVersions = async (): Promise<Version[]> => {
  const dirs = await readDir('.runmc/versions', { dir: Dir.Home })
  const versions = dirs.map(dir => ({ id: dir.name || '?', url: '' }))

  return versions
}

export const installVersion = async (version: Version): Promise<void> => {
  const versionDir = `.runmc/versions/${version.id}`
  await createDir(versionDir, { recursive: true, dir: Dir.Home })

  const r = await fetch(version.url)
  const j = await r.json()

  const url : string = j.downloads.client.url
  await promisified({ cmd: 'downloadFile', url: url, path: `${versionDir}/client.jar` })
}

export const removeVersion = async (version: Version): Promise<void> => {
  const versionDir = `.runmc/versions/${version.id}`
  await removeDir(versionDir, { dir: Dir.Home, recursive: true })
}
