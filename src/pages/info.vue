<script setup lang="ts">
import { version } from '~/../package.json'
</script>

<template>
  <h1 class="text-2xl">
    runmc version {{ version }}
  </h1>
  <div>
    MIT Licensed |
    © 2021 Manuel Quarneti |
    Source code <a href="https://github.com/mq1/runmc" class="underline">here</a>
  </div>
</template>
