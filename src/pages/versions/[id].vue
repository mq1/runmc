<script setup lang="ts">
import { defineProps } from 'vue'

const props = defineProps({
  id: {
    type: String,
    required: true,
  },
})
</script>

<template>
  <div class="flex flex-col gap-y-4 min-w-64">
    <h1 class="text-3xl text-center my-4">
      Version {{ props.id }}
    </h1>
    <h2 class="text-2xl">
      Mod loaders
    </h2>
    <div class="grid grid-cols-2">
      <div>Fabric</div>
      <div>not installed</div>
    </div>
    <h2 class="text-2xl">
      Mods
    </h2>
  </div>
</template>
