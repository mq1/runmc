<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Config } from '~/types'

const { t, availableLocales, locale } = useI18n()

const config = ref<Config>()
const getConfig = () => {
  invoke('get_config')
    .then(c => config.value = c as Config)
    .catch((e: string) => console.error(e))
}
const saveConfig = () => {
  invoke('save_config', { config: config.value })
    .then(getConfig)
    .catch((e: string) => console.error(e))

  locale.value = config.value!.locale
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
    {{ t('nav.settings') }}
  </h1>
  <div v-if="config" class="flex flex-col gap-y-4 min-w-64">
    <label class="flex flex-col">
      <span>{{ t('settings.locale') }}</span>
      <select v-model="config.locale" class="box dark:bg-black">
        <option v-for="l in availableLocales" :key="l">{{ l }}</option>
      </select>
    </label>
    <label class="flex flex-col">
      <span>{{ t('settings.java.path') }}</span>
      <input v-model="config.java.path" type="text" class="box dark:bg-black" />
    </label>
    <label class="flex flex-col">
      <span>{{ t('settings.java.memory') }}</span>
      <input v-model.number="config.java.memory" type="text" class="box dark:bg-black" />
    </label>
  </div>
  <div class="w-full flex justify-end gap-x-2">
    <button class="btn bg-red-500" @click="getDefaultConfig">
      <carbon-reset />
      <span class="ml-2">
        {{ t('settings.reset') }}
      </span>
    </button>
    <button class="btn bg-primary-500" @click="saveConfig">
      <carbon-save />
      <span class="ml-2">
        {{ t('settings.save') }}
      </span>
    </button>
  </div>
</template>
