<script setup lang="ts">
import { defineProps } from 'vue'

const props = defineProps({
  primary: {
    type: Boolean,
    required: false,
  },
  red: {
    type: Boolean,
    required: false,
  },
  yellow: {
    type: Boolean,
    required: false,
  },
  green: {
    type: Boolean,
    required: false,
  },
  blue: {
    type: Boolean,
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
  tiny: {
    type: Boolean,
    required: false,
  },
})
</script>

<template>
  <button
    :class="[
      'flex items-center gap-x-2 rounded-lg shadow-md no-underline focus:outline-none py-2 px-4 min-w-full bg-white text-black dark:bg-black dark:text-white',
      props.primary ? 'bg-purple-500 text-white dark:bg-purple-500' : '',
      props.red? 'bg-red-500 text-white dark:bg-red-500' : '',
      props.yellow? 'bg-yellow-500 text-white dark:bg-yellow-500' : '',
      props.green? 'bg-green-500 text-white dark:bg-green-500' : '',
      props.blue? 'bg-blue-500 text-white dark:bg-blue-500' : '',
      props.center ? 'justify-center' : '',
      props.small ? 'pt-1 pb-1 pl-2 pr-2' : '',
      props.selected ? 'font-bold text-purple-500' : '',
      props.short ? 'min-w-min' : '',
      props.tiny ? 'min-w-min pt-1 pb-1 pl-1 pr-1' : '',
    ]"
  >
    <slot />
  </button>
</template>
