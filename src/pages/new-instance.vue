<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import type { Version } from '~/types'

const { t } = useI18n()
const router = useRouter()

const snapshotsEnabled = ref(false)

const installing = ref(false)

const versions = ref<Version[]>()
const updateVersions = () => {
  invoke('list_available_game_versions')
    .then(v => versions.value = v as Version[])
    .catch((e: string) => console.error(e))
}

const newInstance = (version: Version) => {
  installing.value = true

  invoke('init_instance', {
    instanceName: version.id,
    gameVersion: version,
  })
    .then(() => router.push('/instances'))
    .catch((e: string) => console.log(e))
}

onMounted(updateVersions)
</script>

<template>
  <h1 class="text-3xl text-center">
    {{ t('instances.availableversions') }}
  </h1>

  <div v-if="versions" class="overflow-y-auto h-full w-full flex flex-col items-center gap-y-4">
    <div
      v-for="version in versions.filter(v => v.type === 'release' || (v.type === 'snapshot' && snapshotsEnabled))"
      :key="version.id"
      class="p-2 flex items-center justify-between border-1 rounded-lg shadow-md min-w-sm gap-x-4"
    >
      <carbon-fire v-if="version.type === 'snapshot'" class="text-red-500" />
      <carbon-badge v-if="version.type === 'release'" class="text-green-500" />
      {{ version.type }}
      <span class="font-semibold">{{ version.id }}</span>
      <button class="tiny bg-green-500 text-white" @click="newInstance(version)">
        <carbon-download />
      </button>
    </div>
  </div>

  <div class="w-full flex justify-between">
    <div v-show="!installing" />
    <div v-show="installing" class="flex items-center gap-x-2">
      <carbon-restart class="animate-spin" />
      {{ t('installing') }}
    </div>
    <div class="flex items-center gap-x-2">
      <label>{{ t('instances.showsnapshots') }}</label>
      <input v-model="snapshotsEnabled" type="checkbox" />
    </div>
  </div>
</template>
