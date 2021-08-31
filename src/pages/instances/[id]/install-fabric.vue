<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import type { InstanceInfo } from '~/types'

const { t } = useI18n()

const props = defineProps({
  id: {
    type: String,
    required: true,
  },
})

type Loader = {
  version: string
  stable: boolean
}

const router = useRouter()

const versions = ref<Loader[]>()
const updateVersions = () => {
  invoke('read_instance_info', {
    instanceName: props.id,
  })
    .then((instanceInfo) => {
      invoke('get_fabric_loader_versions', {
        minecraftVersion: (instanceInfo as InstanceInfo).gameVersion,
      })
        .then(v => versions.value = v as Loader[])
        .catch((e: string) => console.error(e))
    })
    .catch((e: string) => console.error(e))
}

const showUnstable = ref(false)

const install = (version: string) => {
  invoke('install_fabric', {
    instanceName: props.id,
    loaderVersion: version,
  })
    .then(() => router.push(`/instances/${props.id}`))
    .catch((e: string) => console.error(e))
}

onMounted(updateVersions)
</script>

<template>
  <h1 class="text-3xl text-center">
    {{ t('instances.fabric', { version: props.id }) }}
  </h1>

  <ul v-if="versions" class="h-full w-full flex flex-col items-center gap-y-4 overflow-y-auto">
    <li v-for="version in versions.filter(v => v.stable || showUnstable)" :key="version.version" class="p-2 border-1 rounded-lg shadow-md w-96 flex justify-between items-center">
      <div class="flex gap-x-2 items-center flex-1">
        <carbon-fire v-if="!version.stable" class="text-red-700" />
        <carbon-badge v-if="version.stable" class="text-green-700" />
        <span>{{ version.stable ? 'stable' : 'unstable' }}</span>
      </div>
      <span class="flex-1">{{ version.version }}</span>
      <button class="bg-green-500 small text-white w-auto" @click="install(version.version)">
        {{ t('install') }}
      </button>
    </li>
  </ul>

  <div class="flex gap-x-2 justify-end items-center w-full">
    <label>{{ t('instances.showunstable') }}</label>
    <input v-model="showUnstable" type="checkbox" />
  </div>
</template>
