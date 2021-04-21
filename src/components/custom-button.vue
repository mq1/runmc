<script setup lang="ts">
import { defineProps } from 'vue'

const props = defineProps({
  color: {
    type: String,
    required: false,
  },
  center: {
    type: Boolean,
    required: false,
  },
  small: {
    type: Boolean,
    required: false,
  },
  selected: {
    type: Boolean,
    required: false,
  },
  short: {
    type: Boolean,
    required: false,
  },
})
</script>

<template>
  <button
    :class="[
      props.color ? `bg-${props.color}-500 text-white` : 'bg-white text-black',
      props.center ? 'justify-center' : '',
      props.small ? 'py-1 px-2' : 'py-2 px-4',
      props.selected ? 'font-bold text-purple-700' : '',
      props.short ? '' : 'min-w-full',
      'flex items-center gap-x-2 rounded-lg shadow-md no-underline focus:outline-none uppercase'
    ]"
  >
    <slot />
  </button>
</template>
