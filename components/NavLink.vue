<script setup lang="ts">
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
        isExactActive && 'bg-primary-700 dark:bg-primary-500 text-white'
      ]"
    >
      <slot />
    </button>
  </router-link>
</template>
