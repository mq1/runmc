<script setup lang="ts">
import { defineProps } from 'vue'

const props = defineProps({
  to: {
    type: String,
    required: true,
  },
})
</script>

<template>
  <router-link class="flex items-center gap-x-2 py-1 px-2 mx-2 bg-white text-black rounded-lg shadow-md" :to="props.to">
    <slot />
  </router-link>
</template>
