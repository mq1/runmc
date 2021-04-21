<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { useRouter } from 'vue-router'
import type { Version } from '~/types'

const router = useRouter()

const snapshotsEnabled = ref(false)

const availableVersions = ref<Version[]>([])
const updateAvailableVersions = () => {
  invoke('list_available_versions')
    .then(v => availableVersions.value = v as Version[])
    .catch((e: string) => console.error(e))
}

const installVersion = (version: Version) => {
  invoke('install_version', { version })
    .then(() => router.push('/versions'))
    .catch((e: string) => console.log(e))
}

onMounted(updateAvailableVersions)
</script>

<template>
  <div class="flex flex-col gap-y-4 min-w-64">
    <h1 class="text-3xl text-center my-4">
      Available Versions
    </h1>
    <div class="overflow-y-auto h-96 w-96 flex flex-col items-center gap-y-4">
      <div
        v-for="version in availableVersions"
        :key="version.id"
        class="p-2 flex items-center justify-between border-1 rounded-lg shadow-md w-72 gap-x-4"
      >
        <heroicons-outline-fire v-if="version.type === 'snapshot'" class="text-red-700" />
        <heroicons-outline-badge-check v-if="version.type === 'release'" class="text-green-700" />
        {{ version.type }}
        <span class="font-semibold">{{ version.id }}</span>
        <InstallButton @click="installVersion(version)" />
      </div>
    </div>
  </div>
  <div class="fixed right-8 bottom-8 flex gap-x-2">
    <Switch v-model="snapshotsEnabled" :class="snapshotsEnabled ? 'bg-teal-900' : 'bg-teal-700'" class="relative inline-flex items-center h-6 rounded-full w-11 cursor-pointer">
      <span class="sr-only">Enable snapshots</span>
      <span
        :class="snapshotsEnabled ? 'translate-x-6' : 'translate-x-1'"
        class="inline-block w-4 h-4 transform bg-white rounded-full"
      />
    </Switch>
  </div>
</template>
