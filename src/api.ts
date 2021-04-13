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

interface Library {
  downloads: {
    artifact: {
      path: string,
      url: string
    },
    classifiers?: {
      'natives-linux': {
        path: string,
        url: string
      },
      'natives-macos': {
        path: string,
        url: string
      },
      'natives-windows': {
        path: string,
        url: string
      }
    }
  }
}

const downloadLibraries = async (basePath: string, libraries: Library[]) => {
  // find platform (os)
  let platform: 'natives-windows' | 'natives-macos' | 'natives-linux'
  if (navigator.appVersion.indexOf('Win') !== -1) {
    platform = 'natives-windows'
  } else if (navigator.appVersion.indexOf('Mac') !== -1) {
    platform = 'natives-macos'
  } else {
    platform = 'natives-linux'
  }

  for (const library of libraries) {
    await promisified({
      cmd: 'downloadFile',
      url: library.downloads.artifact.url,
      path: `${basePath}/libraries/${library.downloads.artifact.path}`
    })

    if (library.downloads.classifiers) {
      await promisified({
        cmd: 'downloadFile',
        url: library.downloads.classifiers[platform].url,
        path: `${basePath}/libraries/${library.downloads.classifiers[platform].path}`
      })
    }
  }
}

const downloadAssets = async (basePath: string, assetIndexURL: string) => {
  const r2 = await fetch(assetIndexURL)
  const j2: {
    objects: {
      [id: string]: {
        hash: string
      }
    }
  } = await r2.json()

  for (const hash of Object.entries(j2.objects).map(([key, value]) => value.hash)) {
    await promisified({
      cmd: 'downloadFile',
      url: `https://resources.download.minecraft.net/${hash.substring(0, 2)}/${hash}`,
      path: `${basePath}/assets/objects/${hash.substring(0, 2)}/${hash}`
    })
  }
}

export const installVersion = async (version: Version): Promise<void> => {
  const versionDir = `.runmc/versions/${version.id}`
  await createDir(versionDir, { recursive: true, dir: Dir.Home })

  const r = await fetch(version.url)
  const j: {
    downloads: {
      client: {
        url: string
      }
    },
    libraries: Library[],
    assetIndex: {
      url: string
    }
  } = await r.json()

  await Promise.all([
    promisified({ cmd: 'downloadFile', url: j.downloads.client.url, path: `${versionDir}/client.jar` }),
    downloadLibraries(versionDir, j.libraries),
    downloadAssets(versionDir, j.assetIndex.url)
  ])
}

export const removeVersion = async (version: Version): Promise<void> => {
  const versionDir = `.runmc/versions/${version.id}`
  await removeDir(versionDir, { dir: Dir.Home, recursive: true })
  console.log(`deleted version ${version.id}`)
}
