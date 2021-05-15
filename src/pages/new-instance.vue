<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { useRouter } from 'vue-router'
import { Switch } from '@headlessui/vue'
import { useI18n } from 'vue-i18n'
import type { Version } from '~/types'

const { t } = useI18n()
const router = useRouter()

const snapshotsEnabled = ref(false)

const installing = ref(false)

const versions = ref<Version[]>([])
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
  <div class="overflow-y-auto h-full w-full flex flex-col items-center gap-y-4">
    <div
      v-for="version in versions.filter(v => v.type === 'release' || (v.type === 'snapshot' && snapshotsEnabled))"
      :key="version.id"
      class="p-2 flex items-center justify-between border-1 rounded-lg shadow-md w-72 gap-x-4"
    >
      <carbon-fire v-if="version.type === 'snapshot'" class="text-red-700" />
      <carbon-badge v-if="version.type === 'release'" class="text-green-700" />
      {{ version.type }}
      <span class="font-semibold">{{ version.id }}</span>
      <button class="btn tiny bg-green-500" @click="newInstance(version)">
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
    <div class="flex gap-x-2">
      <span>{{ t('instances.showsnapshots') }}</span>
      <Switch v-model="snapshotsEnabled" :class="snapshotsEnabled ? 'bg-teal-900' : 'bg-teal-700'" class="relative inline-flex items-center h-6 rounded-full w-11 cursor-pointer focus:outline-none">
        <span class="sr-only">{{ t('instances.showsnapshots') }}</span>
        <span
          :class="snapshotsEnabled ? 'translate-x-6' : 'translate-x-1'"
          class="inline-block w-4 h-4 transform bg-white rounded-full"
        />
      </Switch>
    </div>
  </div>
</template>
