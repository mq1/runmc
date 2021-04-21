<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { ref, onMounted } from 'vue'
import type { Config } from '~/types'

const config = ref<Config>({ java_path: '', java_memory_mb: 0 })
const getConfig = () => {
  invoke('get_config')
    .then(c => config.value = c as Config)
    .catch((e: string) => console.error(e))
}
const saveConfig = () => {
  invoke('save_config', { config: config.value })
    .then(getConfig)
    .catch((e: string) => console.error(e))
}
const getDefaultConfig = () => {
  invoke('get_default_config')
    .then(c => config.value = c as Config)
    .catch((e: string) => console.error(e))
}

onMounted(getConfig)
</script>

<template>
  <h1 class="text-3xl text-center my-4">
    Settings
  </h1>
  <div class="flex flex-col gap-y-4">
    <label class="flex flex-col">
      <span>Java Path</span>
      <input v-model="config.java_path" type="text" class="rounded-lg border-gray-300 shadow-md dark:bg-black" />
    </label>
    <label class="flex flex-col">
      <span>Java allocated memory (in MB)</span>
      <input v-model.number="config.java_memory_mb" type="number" class="rounded-lg border-gray-300 shadow-md dark:bg-black" />
    </label>
  </div>
  <div class="w-full flex justify-end gap-x-2">
    <CustomButton color="red" short @click="getDefaultConfig">
      <heroicons-outline-refresh />
      Reset to defaults
    </CustomButton>
    <CustomButton color="purple" short @click="saveConfig">
      <heroicons-outline-save />
      Save
    </CustomButton>
  </div>
</template>
