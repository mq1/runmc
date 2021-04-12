<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Dir, createDir, readDir } from 'tauri/api/fs'
import { promisified } from 'tauri/api/tauri'

interface Version {
  id: string,
  type?: string,
  url: string
}

const installedVersions = ref<Version[]>([])
const updateInstalledVersions = () => {
  readDir('.runmc/versions', { dir: Dir.Home })
    .then(versions => versions.forEach(version => {
      installedVersions.value.push({ id: version.name || '?', url: '' })
    }))
}

const availableVersions = ref<Version[]>([])
const updateAvailableVersions = () => {
  fetch('https://launchermeta.mojang.com/mc/game/version_manifest.json')
    .then(r => r.json())
    .then(j => { availableVersions.value = j.versions })
}

const installVersion = (version: Version) => {
  const versionDir = `.runmc/versions/${version.id}`

  createDir(versionDir, { recursive: true, dir: Dir.Home })
  fetch(version.url)
    .then(r => r.json())
    .then(j => {
      const url : string = j.downloads.client.url
      promisified({ cmd: 'downloadFile', url: url, path: `${versionDir}/client.jar` })
    })
}

onMounted(() => {
  updateInstalledVersions()
  updateAvailableVersions()
})
</script>

<template>
  <div class="flex justify-between gap-x-16">
    <div v-show="installedVersions.length">
      <h1 class="text-3xl text-center mb-8">Installed Versions</h1>
      <div class="overflow-y-auto h-64 flex flex-col divide-y">
        <div class="p-2 flex justify-between" v-for="version in installedVersions" :key="version.id">
          {{ version.id }}
          <button class="px-2 py-1 bg-red-500 text-white rounded-full">Remove 🗑</button>
        </div>
      </div>
    </div>
    <div>
      <h1 class="text-3xl text-center mb-8">Available Versions</h1>
      <div class="overflow-y-auto h-64 flex flex-col divide-y">
        <div class="p-2 flex justify-between" v-for="version in availableVersions" :key="version.id">
          {{ version.type === 'release' ? '✅' : '🔥' }} {{ version.type }} {{ version.id }}
          <button class="px-2 py-1 bg-green-500 text-white rounded-full" @click="installVersion(version)">Install ➕</button>
        </div>
      </div>
    </div>
  </div>
</template>
