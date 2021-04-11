<script setup lang="ts">
import { ref } from 'vue'

const isPlayerSelectorDropdownOpen = ref(false)
</script>

<template>
  <div class="p-4 flex flex-col justify-between h-screen">
    <nav class="flex justify-between">
      <div>
        <router-link to="/">🏠</router-link>
      </div>
      <div class="flex gap-x-4">
        <button class="flex gap-x-2 w-28" @click="isPlayerSelectorDropdownOpen = !isPlayerSelectorDropdownOpen">
          username
          <img class="h-6" src="@/assets/logo.png" />
        </button>
        <div class="absolute mt-8 w-28 border-2 rounded-3xl flex flex-col items-center" v-show="isPlayerSelectorDropdownOpen">
          <router-link class="p-1" to="/players" @click="isPlayerSelectorDropdownOpen = false">+</router-link>
        </div>
        <router-link to="/settings">⚙</router-link>
        <router-link to="/about">?</router-link>
      </div>
    </nav>

    <main class="m-auto">
      <router-view />
    </main>

    <footer class="text-center min-h-max">
      MIT Licensed | © 2021 Manuel Quarneti
    </footer>
  </div>
</template>
