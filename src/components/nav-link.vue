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
  <router-link
    v-slot="{ isExactActive }"
    :to="props.to"
  >
    <button
      :class="[
        'small',
        isExactActive && 'font-bold text-primary-500 dark:text-primary-500'
      ]"
    >
      <slot />
    </button>
  </router-link>
</template>
