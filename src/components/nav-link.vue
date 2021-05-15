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
    <div
      :class="[
        'btn-white small',
        isExactActive ? 'font-bold text-primary-500 dark:text-primary-500' : 'text-black dark:text-white'
      ]"
    >
      <slot />
    </div>
  </router-link>
</template>
