import { invoke } from '@tauri-apps/api/tauri'
import { appDir } from '@tauri-apps/api/path'

export type GameVersion = {
  id: string
  type: 'release' | 'snapshot'
  url: string
}

type Artifact = {
  path: string
  url: string
}

export type Library = {
  downloads: {
    artifact: Artifact
    classifiers?: {
      [native: string]: Artifact
    }
  }
  natives?: {
    linux?: string
    osx?: string
    windows?: string
  }
  rules?: {
    action: string
    os?: {
      name: string
    }
  }[]
}

export type GameVersionInfo = {
  assetIndex: {
    id: string
    url: string
  }
  downloads: {
    client: Artifact
  }
  libraries: Library[]
}

export const fetchGameVersions = () =>
  fetch('https://launchermeta.mojang.com/mc/game/version_manifest.json')
    .then(response => response.json() as Promise<{ versions: GameVersion[] }>)
    .then(data => data.versions)

export const downloadGameVersionMetaData = async({ id, url }: GameVersion) => {
  const dir = await appDir()
  const path = `${dir}/meta/net.minecraft/${id}.json`

  await invoke('download_file', { url, path })
}
