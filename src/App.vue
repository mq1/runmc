<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { appDir as getAppDir } from '@tauri-apps/api/path'
import { createDir, BaseDirectory } from '@tauri-apps/api/fs'
import type { Config } from '~/types'

const { locale } = useI18n()

onBeforeMount(async() => {
  // create appDir if not present
  const appDir = await getAppDir()
  await createDir(appDir, { recursive: true })
  console.log({ appDir })

  // create instances dir
  await createDir('instances', { dir: BaseDirectory.App, recursive: true })

  // create meta dir
  await createDir('meta', { dir: BaseDirectory.App, recursive: true })

  // set locale
  const config: Config = await invoke('read_config')
  locale.value = config.locale
  console.log({ locale: locale.value })
})
</script>

<template>
  <router-view />
</template>
