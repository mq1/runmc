<script setup lang="ts">
import { appDir } from '@tauri-apps/api/path'
import { createDir, BaseDirectory } from '@tauri-apps/api/fs'
import { read } from '~/logic/config'

const { locale } = useI18n()

onBeforeMount(async() => {
  // create appDir if not present
  const appdir = await appDir()
  await createDir(appdir, { recursive: true })
  console.log({ appDir: appdir })

  // create instances dir
  await createDir('instances', { dir: BaseDirectory.App, recursive: true })

  // set locale
  const config = await read()
  locale.value = config.locale
  console.log({ locale: locale.value })
})
</script>

<template>
  <router-view />
</template>
