<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

interface Version {
  id: string
  type: string
  url: string
}

const installedVersions = ref<string[]>([])
const updateInstalledVersions = () => {
  invoke('list_versions')
    .then(v => installedVersions.value = v as string[])
    .catch((e: string) => console.error(e))
}

const availableVersions = ref<Version[]>([])
const updateAvailableVersions = () => {
  invoke('list_available_versions')
    .then(v => availableVersions.value = v as Version[])
    .catch((e: string) => console.error(e))
}

const installVersion = (version: Version) => {
  invoke('install_version', { version })
    .then(updateInstalledVersions)
    .catch((e: string) => console.log(e))
}

const removeVersion = (version: string) => {
  invoke('remove_version', { version })
    .then(updateInstalledVersions)
    .catch((e: string) => console.error(e))
}

onMounted(() => {
  updateInstalledVersions()
  updateAvailableVersions()
})
</script>

<template>
  <div class="flex gap-x-8">
    <div v-show="installedVersions.length" class="flex-1 min-w-72">
      <h1 class="text-3xl text-center mb-8">
        Installed Versions
      </h1>
      <div class="border-2 rounded-3xl p-2 overflow-y-auto h-64 flex flex-col divide-y">
        <div
          v-for="version in installedVersions"
          :key="version"
          class="p-2 flex items-center justify-between"
        >
          {{ version }}
          <RemoveButton @click="removeVersion(version)" />
        </div>
      </div>
    </div>
    <div class="flex-1 min-w-72">
      <h1 class="text-3xl text-center mb-8">
        Available Versions
      </h1>
      <div class="border-2 rounded-3xl p-2 overflow-y-auto h-64 flex flex-col divide-y">
        <div
          v-for="version in availableVersions"
          :key="version.id"
          class="p-2 flex items-center justify-between"
        >
          <heroicons-outline-fire v-if="version.type === 'snapshot'" class="text-red-700" />
          <heroicons-outline-badge-check v-if="version.type === 'release'" class="text-green-700" />
          {{ version.type }}
          <span class="font-semibold">{{ version.id }}</span>
          <InstallButton @click="installVersion(version)" />
        </div>
      </div>
    </div>
  </div>
</template>
