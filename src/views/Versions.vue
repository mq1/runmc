<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getAvailableVersions, getInstalledVersions, installVersion, removeVersion } from '@/api'
import type { Version } from '@/api'

const installedVersions = ref<Version[]>([])
const updateInstalledVersions = () => {
  getInstalledVersions().then(versions => { installedVersions.value = versions })
}

const availableVersions = ref<Version[]>([])
const updateAvailableVersions = () => {
  getAvailableVersions().then(versions => { availableVersions.value = versions })
}

const install = (version: Version) => {
  installVersion(version).then(updateInstalledVersions)
}

const remove = (version: Version) => {
  removeVersion(version).then(updateInstalledVersions)
}

onMounted(() => {
  updateInstalledVersions()
  updateAvailableVersions()
})
</script>

<template>
  <div class="flex gap-x-8">
    <div class="flex-1" v-show="installedVersions.length">
      <h1 class="text-3xl text-center mb-8">Installed Versions</h1>
      <div class="border-2 rounded-3xl p-2 overflow-y-auto h-64 flex flex-col divide-y">
        <div class="p-2 flex justify-between" v-for="version in installedVersions" :key="version.id">
          {{ version.id }}
          <button class="px-2 py-1 bg-red-500 text-white rounded-full" @click="remove(version)">Remove 🗑</button>
        </div>
      </div>
    </div>
    <div class="flex-1">
      <h1 class="text-3xl text-center mb-8">Available Versions</h1>
      <div class="border-2 rounded-3xl p-2 overflow-y-auto h-64 flex flex-col divide-y">
        <div class="p-2 flex justify-between" v-for="version in availableVersions" :key="version.id">
          {{ version.type === 'release' ? '✅' : '🔥' }} {{ version.type }} {{ version.id }}
          <button class="px-2 py-1 bg-green-500 text-white rounded-full" @click="install(version)">Install ➕</button>
        </div>
      </div>
    </div>
  </div>
</template>
