import { invoke } from '@tauri-apps/api/tauri'

export interface Version {
  id: string
  type?: string
  url: string
}

interface Library {
  downloads: {
    artifact: {
      path: string
      url: string
    }
    classifiers?: {
      'natives-linux': {
        path: string
        url: string
      }
      'natives-macos': {
        path: string
        url: string
      }
      'natives-windows': {
        path: string
        url: string
      }
    }
  }
}

const downloadLibraries = async(basePath: string, libraries: Library[]) => {
  // find platform (os)
  let platform: 'natives-windows' | 'natives-macos' | 'natives-linux'
  if (navigator.appVersion.includes('Win'))
    platform = 'natives-windows'

  else if (navigator.appVersion.includes('Mac'))
    platform = 'natives-macos'

  else
    platform = 'natives-linux'

  for (const library of libraries) {
    const fileName = library.downloads.artifact.path.substring(
      library.downloads.artifact.path.lastIndexOf('/') + 1,
    )
    await invoke('download_file', {
      url: library.downloads.artifact.url,
      path: `${basePath}/libraries/${fileName}`,
    })

    if (library.downloads.classifiers) {
      const fileName = library.downloads.classifiers[platform].path.substring(
        library.downloads.artifact.path.lastIndexOf('/') + 1,
      )
      await invoke('download_file', {
        url: library.downloads.classifiers[platform].url,
        path: `${basePath}/libraries/${fileName}`,
      })
    }
  }
}

const downloadAssets = async(basePath: string, assetIndexURL: string) => {
  const r2 = await fetch(assetIndexURL)
  const j2: {
    objects: {
      [id: string]: {
        hash: string
      }
    }
  } = await r2.json()

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  for (const hash of Object.entries(j2.objects).map(([key, value]) => value.hash)) {
    await invoke('download_file', {
      url: `https://resources.download.minecraft.net/${hash.substring(0, 2)}/${hash}`,
      path: `${basePath}/assets/objects/${hash.substring(0, 2)}/${hash}`,
    })
  }
}

export const installVersion = async(version: Version): Promise<void> => {
  const versionDir = `versions/${version.id}`

  const r = await fetch(version.url)
  const j: {
    downloads: {
      client: {
        url: string
      }
    }
    libraries: Library[]
    assetIndex: {
      url: string
    }
  } = await r.json()

  await Promise.all([
    invoke('download_file', {
      url: j.downloads.client.url,
      path: `${versionDir}/libraries/client.jar`,
    }),
    downloadLibraries(versionDir, j.libraries),
    downloadAssets(versionDir, j.assetIndex.url),
  ])
}
