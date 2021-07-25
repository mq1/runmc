export type GameVersion = {
  id: string
  type: 'release' | 'snapshot'
  url: string
}

export const fetchGameVersions = () =>
  fetch('https://launchermeta.mojang.com/mc/game/version_manifest.json')
    .then(response => response.json() as Promise<{ versions: GameVersion[] }>)
    .then(data => data.versions)
