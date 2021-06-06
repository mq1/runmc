<script setup lang="ts">
import { onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/tauri'
import type { Config } from '~/types'

const { t, locale } = useI18n()
const updateLocale = () => {
  invoke('get_config')
    .then(config => locale.value = (config as Config).locale)
    .catch((e: string) => console.error(e))
}

onMounted(updateLocale)
</script>

<template>
  <div class="flex flex justify-between h-screen dark:bg-black dark:text-white p-4">
    <nav class="flex flex-col p-4 w-44 bg-primary-500 dark:bg-primary-700 rounded-xl shadow-2xl justify-between">
      <div class="flex flex-col gap-y-2">
        <NavLink to="/">
          <carbon-home /> {{ t('nav.index') }}
        </NavLink>
        <NavLink to="/accounts">
          <carbon-user-multiple /> {{ t('nav.accounts') }}
        </NavLink>
        <NavLink to="/instances">
          <carbon-layers /> {{ t('nav.instances') }}
        </NavLink>
      </div>
      <div class="flex flex-col gap-y-2">
        <NavLink to="/settings">
          <carbon-settings-adjust /> {{ t('nav.settings') }}
        </NavLink>
        <NavLink to="/info">
          <carbon-information /> {{ t('nav.info') }}
        </NavLink>
      </div>
    </nav>
    <main class="p-4 flex-1 flex flex-col items-center justify-between gap-y-8 min-w-xs max-w-screen max-h-screen overflow-auto">
      <router-view />
    </main>
  </div>
</template>
