<script setup lang="ts">
import { ref } from 'vue'

const isVersionSelectorDropdownOpen = ref(false)
</script>

<template>
  <h1 class="text-5xl text-center">runmc</h1>
  <div class="my-8">
    <button class="border-2 w-40 rounded-l-full py-2 px-4" @click="isVersionSelectorDropdownOpen = !isVersionSelectorDropdownOpen">Select version ↓</button>
    <div class="absolute border-2 rounded-3xl mt-2 w-40 flex flex-col items-center" v-show="isVersionSelectorDropdownOpen">
      <router-link class="p-2" to="/versions">+</router-link>
    </div>
    <button class="text-white bg-blue-500 border-2 border-blue-500 rounded-r-full py-2 px-4">▶</button>
  </div>
</template>
