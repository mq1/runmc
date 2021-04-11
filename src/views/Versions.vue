<script setup lang="ts">
import { ref, onMounted } from 'vue'

const availableVersions = ref<{ id: string, type: string }[]>([])
const updateAvailableVersions = () => {
  fetch('https://launchermeta.mojang.com/mc/game/version_manifest.json')
    .then(r => r.json())
    .then(j => { availableVersions.value = j.versions })
}

onMounted(updateAvailableVersions)
</script>

<template>
  <div>
    <h1 class="text-3xl text-center mb-8">Available Versions</h1>
    <div class="overflow-y-auto h-56 flex flex-col divide-y">
      <div class="p-2 flex justify-between" v-for="version in availableVersions" :key="version.id">
        {{ version.type === 'release' ? '✅' : '🔥' }} {{ version.type }} {{ version.id }}
        <button class="px-2 py-1 bg-green-500 text-white rounded-full">Install ➕</button>
      </div>
    </div>
  </div>
</template>
