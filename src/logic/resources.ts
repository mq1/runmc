import { invoke } from '@tauri-apps/api/tauri'
import { appDir } from '@tauri-apps/api/path'

export const downloadAssetIndex = async(id: string, url: string) => {
  const dir = await appDir()
  const path = `${dir}/assets/indexes/${id}.json`

  await invoke('download_file', { url, path })
}

export const downloadObject = async(hash: string) => {
  const first2 = hash.substring(0, 2)
  const dir = await appDir()

  const url = `http://resources.download.minecraft.net/${first2}/${hash}`
  const path = `${dir}/assets/objects/${first2}/${hash}`

  await invoke('download_file', { url, path })
}
