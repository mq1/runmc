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
        isExactActive && 'font-bold bg-transparent border-2 border-white text-white'
      ]"
    >
      <slot />
    </button>
  </router-link>
</template>
