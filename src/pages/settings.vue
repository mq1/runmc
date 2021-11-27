<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import type { Config } from '~/types'

const { t, availableLocales, locale } = useI18n()

const config: Ref<Config> = ref()

const readConfig = () =>
  invoke('read_config')
    .then(data => config.value = data as Config)
    .catch(e => console.error(e))

const writeConfig = () =>
  invoke('write_config', { config: config.value })
    .then(() => locale.value = config.value.locale)
    .catch(e => console.error(e))

const getDefaultConfig = () =>
  invoke('get_default_config')
    .then(data => config.value = data as Config)
    .catch(e => console.error(e))

onMounted(readConfig)
</script>

<template>
  <h1 class="text-3xl text-center">
    {{ t('nav.settings') }}
  </h1>
  <div v-if="config" class="flex flex-col gap-y-4 min-w-xs">
    <label class="flex flex-col">
      <span>{{ t('settings.locale') }}</span>
      <select v-model="config.locale">
        <option v-for="l in availableLocales" :key="l">{{ l }}</option>
      </select>
    </label>
    <label class="flex flex-col">
      <span>{{ t('settings.java.path') }}</span>
      <input v-model="config.java.path" type="text" />
    </label>
    <label class="flex flex-col">
      <span>{{ t('settings.java.memory') }}</span>
      <input v-model.number="config.java.memory" type="text" />
    </label>
  </div>
  <div class="w-full flex justify-end gap-x-2">
    <button class="w-auto bg-red-500 text-white flex gap-x-2" @click="getDefaultConfig">
      <carbon-reset />
      <span>
        {{ t('settings.reset') }}
      </span>
    </button>
    <button class="w-auto bg-primary-500 text-white flex gap-x-2" @click="writeConfig">
      <carbon-save />
      <span>
        {{ t('settings.save') }}
      </span>
    </button>
  </div>
</template>
