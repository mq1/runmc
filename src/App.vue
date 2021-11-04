<script setup lang="ts">
import { appDir } from '@tauri-apps/api/path'
import { createDir } from '@tauri-apps/api/fs'
import { read } from '~/logic/config'

const { locale } = useI18n()

onBeforeMount(async() => {
  // create appDir if not present
  const d = await appDir()
  await createDir(d, { recursive: true })
  console.log({ appDir: d })

  // set locale
  const config = await read()
  locale.value = config.locale
  console.log({ locale: locale.value })
})
</script>

<template>
  <router-view />
</template>
