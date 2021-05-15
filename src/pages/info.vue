<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { version } from '~/../package.json'

const { t } = useI18n()
</script>

<template>
  <h1 class="text-2xl">
    {{ t('info.version', { version }) }}
  </h1>
  <div>
    MIT Licensed |
    © 2021 Manuel Quarneti |
    {{ t('info.sourcecode') }} <a href="https://github.com/mq1/runmc" class="underline">{{ t('info.here') }}</a>
  </div>
</template>
