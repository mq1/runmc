<script setup lang="ts">
import { read, write, getDefaultConfig } from '~/logic/config'

const { t, availableLocales, locale } = useI18n()

const config = ref(getDefaultConfig())

const getConfig = async() =>
  config.value = await read()

const saveConfig = () =>
  write(config.value)
    .then(() => locale.value = config.value.locale)

const loadDefaultConfig = () =>
  config.value = getDefaultConfig()

onMounted(getConfig)
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
    <button class="w-auto bg-red-500 text-white flex gap-x-2" @click="loadDefaultConfig">
      <carbon-reset />
      <span>
        {{ t('settings.reset') }}
      </span>
    </button>
    <button class="w-auto bg-primary-500 text-white flex gap-x-2" @click="saveConfig">
      <carbon-save />
      <span>
        {{ t('settings.save') }}
      </span>
    </button>
  </div>
</template>
