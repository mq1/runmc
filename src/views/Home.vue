<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getInstalledVersions, executeVersion } from '@/api'
import type { Version } from '@/api'

const versions = ref<Version[]>([])
const updateVersions = () => {
  getInstalledVersions().then(v => { versions.value = v })
}

const isVersionSelectorDropdownOpen = ref(false)

const selectedVersion = ref<Version>({ id: 'Select version ↓', url: '' })
const selectVersion = (version: Version) => {
  selectedVersion.value = version
  isVersionSelectorDropdownOpen.value = false
}

const accessToken = ref('')

onMounted(updateVersions)
</script>

<template>
  <h1 class="text-5xl text-center">runmc</h1>
  <div class="my-8">
    <button class="border-2 w-40 rounded-l-full py-2 px-4" @click="isVersionSelectorDropdownOpen = !isVersionSelectorDropdownOpen">{{ selectedVersion.id }}</button>
    <div class="absolute border-2 rounded-3xl mt-2 w-40 flex flex-col items-center divide-y bg-white" v-show="isVersionSelectorDropdownOpen">
      <button class="p-2 w-full" v-for="version in versions" :key="version.id" @click="selectVersion(version)">{{ version.id }}</button>
      <router-link class="p-2 w-full text-center" to="/versions">+</router-link>
    </div>
    <button class="text-white bg-blue-500 border-2 border-blue-500 rounded-r-full py-2 px-4" @click="executeVersion(selectedVersion, accessToken)">▶</button>
  </div>
  <div>
    Access token
    <input class="border-2 rounded-full p-2" v-model="accessToken">
  </div>
</template>
