<script setup lang="ts">
import { appDir } from '@tauri-apps/api/path'
import { createDir } from '@tauri-apps/api/fs'
import { read } from '~/logic/config'

const { locale } = useI18n()

onBeforeMount(async() => {
  const d = await appDir()
  await createDir(d, { recursive: true })

  const config = await read()
  locale.value = config.locale
})
</script>

<template>
  <router-view />
</template>
