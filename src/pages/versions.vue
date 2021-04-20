<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const installedVersions = ref<string[]>([])
const updateInstalledVersions = () => {
  invoke('list_versions')
    .then(v => installedVersions.value = v as string[])
    .catch((e: string) => console.error(e))
}

const removeVersion = (version: string) => {
  invoke('remove_version', { version })
    .then(updateInstalledVersions)
    .catch((e: string) => console.error(e))
}

onMounted(updateInstalledVersions)
</script>

<template>
  <div class="flex flex-col gap-y-4 min-w-64">
    <h1 class="text-3xl text-center my-4">
      Installed Versions
    </h1>
    <div
      v-for="version in installedVersions"
      :key="version"
      class="p-2 flex items-center justify-between border-1 rounded-lg shadow-md"
    >
      {{ version }}
      <RemoveButton @click="removeVersion(version)" />
    </div>
    <CustomButton as="router-link" to="/add-version" color="purple" center>
      Add a version
    </CustomButton>
  </div>
</template>
