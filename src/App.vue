<script setup lang="ts">
import { appDir as getAppDir } from '@tauri-apps/api/path'
import { createDir, BaseDirectory } from '@tauri-apps/api/fs'
import { read } from '~/logic/config'

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
  const config = await read()
  locale.value = config.locale
  console.log({ locale: locale.value })
})
</script>

<template>
  <router-view />
</template>
