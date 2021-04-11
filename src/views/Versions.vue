<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Dir, createDir, writeBinaryFile } from 'tauri/api/fs'
import http from 'tauri/api/http'
import type { HttpOptions } from 'tauri/api/http'

interface Version {
  id: string,
  type: string,
  url: string
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
      console.log(`downloading ${j.downloads.client.url}`)

      // extremely slow, TODO optimize
      http.get(j.downloads.client.url as string, { responseType: http.ResponseType.Binary } as HttpOptions)
        .then(r => {
          console.log('writing client.jar')
          const bytes: Uint8Array = JSON.parse(r as string)
          writeBinaryFile({ path: `${versionDir}/client.jar`, contents: bytes }, { dir: Dir.Home })
          console.log('client.jar written')
        })
    })
}

onMounted(updateAvailableVersions)
</script>

<template>
  <div>
    <h1 class="text-3xl text-center mb-8">Available Versions</h1>
    <div class="overflow-y-auto h-56 flex flex-col divide-y">
      <div class="p-2 flex justify-between" v-for="version in availableVersions" :key="version.id">
        {{ version.type === 'release' ? '✅' : '🔥' }} {{ version.type }} {{ version.id }}
        <button class="px-2 py-1 bg-green-500 text-white rounded-full" @click="installVersion(version)">Install ➕</button>
      </div>
    </div>
  </div>
</template>
