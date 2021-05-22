<script setup lang="ts">
import { onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/tauri'
import type { Config } from '~/types'

const { t, locale } = useI18n()
const updateLocale = () => {
  invoke('get_config')
    .then(config => locale.value = (config as Config).locale)
}

onMounted(updateLocale)
</script>

<template>
  <div class="flex flex justify-between h-screen dark:bg-black dark:text-white">
    <nav class="flex flex-col p-2 w-40 bg-primary-500 dark:bg-primary-700 justify-between">
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
    <main class="flex-1 flex flex-col items-center justify-between gap-y-8 m-8 max-w-screen max-h-screen overflow-auto">
      <transition
        enter-active-class="transition duration-100 ease-in-out"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition duration-100 ease-in-out"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <router-view />
      </transition>
    </main>
  </div>
</template>
