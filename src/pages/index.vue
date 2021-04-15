<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getInstalledVersions, executeVersion } from '~/api'

const versions = ref<string[]>([])
const getVersions = () => {
  getInstalledVersions().then(v => versions.value = v)
}

const isVersionSelectorDropdownOpen = ref(false)
const toggleDropdown = () => {
  isVersionSelectorDropdownOpen.value = !isVersionSelectorDropdownOpen.value
}

const selectedVersion = ref('Select version ↓')
const selectVersion = (version: string) => {
  selectedVersion.value = version
  isVersionSelectorDropdownOpen.value = false
}

const accessToken = ref('')

onMounted(getVersions)
</script>

<template>
  <h1 class="text-5xl">
    runmc
  </h1>

  <div class="my-8 flex">
    <button class="border-2 w-40 rounded-l-full py-2 px-4" @click="toggleDropdown">
      {{ selectedVersion }}
    </button>
    <div
      v-show="isVersionSelectorDropdownOpen"
      class="absolute border-2 rounded-3xl mt-12 w-40 flex flex-col items-center divide-y bg-white"
    >
      <button v-for="version in versions" :key="version" class="p-2 w-full text-center" @click="selectVersion(version)">
        {{ version }}
      </button>
      <a class="p-2 w-full text-center" href="/versions">+</a>
    </div>
    <button
      class="text-white bg-blue-500 border-2 border-blue-500 rounded-r-full py-2 px-4"
      @click="executeVersion(selectedVersion, accessToken)"
    >
      ▶
    </button>
  </div>

  <div>
    Access token
    <input v-model="accessToken" class="border-2 rounded-full p-2" />
  </div>
</template>
