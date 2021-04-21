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
  tiny: {
    type: Boolean,
    required: false,
  },
})
</script>

<template>
  <button
    :class="[
      'flex items-center gap-x-2 rounded-lg shadow-md no-underline focus:outline-none py-2 px-4 min-w-full',
      props.color ? `bg-${props.color}-500 text-white` : 'bg-white text-black',
      props.center ? 'justify-center' : '',
      props.small ? 'pt-1 pb-1 pl-2 pr-2' : '',
      props.selected ? 'font-bold text-purple-700' : '',
      props.short ? 'min-w-min' : '',
      props.tiny ? 'min-w-min pt-1 pb-1 pl-1 pr-1' : '',
    ]"
  >
    <slot />
  </button>
</template>
