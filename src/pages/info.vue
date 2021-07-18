<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { getVersion } from '@tauri-apps/api/app'

const { t } = useI18n()

const version = ref<string>()
const updateVersion = () => {
  getVersion().then(v => version.value = v)
}

onMounted(updateVersion)
</script>

<template>
  <h1 class="text-2xl">
    {{ t('info.version', { version }) }}
  </h1>
  <footer class="flex justify-center divide-x-2">
    <div class="px-2 flex items-center">
      MIT Licensed
    </div>
    <div class="px-2 flex items-center">
      © 2021 Manuel Quarneti
    </div>
    <a class="px-2 flex items-center" href="https://github.com/mq1/mq1.eu">
      <carbon-logo-github />
    </a>
  </footer>
</template>
