import { invoke } from '@tauri-apps/api/tauri'
import { appDir } from '@tauri-apps/api/path'
import type { Library } from './launchermeta'

const getOS = () => {
  if (navigator.appVersion.includes('Win')) return 'windows'
  if (navigator.appVersion.includes('Mac')) return 'osx'
  if (navigator.appVersion.includes('Linux')) return 'linux'

  return undefined
}

export const downloadLibrary = async(lib: Library) => {
  const os = getOS()

  if (os === undefined)
    return

  if (
    lib.rules === undefined
    || lib.rules.filter(rule => rule.os?.name === os && rule.action === 'allow').length > 0
    || !(lib.rules.filter(rule => rule.os?.name === os && rule.action === 'disallow').length > 0)
  ) {
    const dir = await appDir()

    // download jar
    const { url, path } = lib.downloads.artifact
    const fullPath = `${dir}/libraries/${path}`
    await invoke('download_file', { url, path: fullPath })

    // download jar (native)
    if (lib.natives !== undefined && lib.natives[os] !== undefined) {
      const { url, path } = lib.downloads.classifiers![lib.natives[os]!]
      const fullPath = `${dir}/libraries/${path}`
      await invoke('download_file', { url, path: fullPath })
    }
  }
}
