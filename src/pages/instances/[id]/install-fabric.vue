<script setup lang="ts">
import { defineProps, onMounted, ref } from 'vue'
import { Switch } from '@headlessui/vue'
import { invoke } from '@tauri-apps/api/tauri'
import { useRouter } from 'vue-router'

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

const versions = ref<Loader[]>([])
const updateVersions = () => {
  invoke('get_fabric_loader_versions', {
    instanceName: props.id,
  })
    .then(v => versions.value = v as Loader[])
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
    Fabric for Minecraft {{ props.id }}
  </h1>
  <ul class="h-full w-full flex flex-col items-center gap-y-4 overflow-y-auto">
    <li v-for="version in versions.filter(v => v.stable || showUnstable)" :key="version.version" class="p-2 border-1 rounded-lg shadow-md w-96 flex justify-between items-center">
      <div class="flex gap-x-2 items-center flex-1">
        <carbon-fire v-if="!version.stable" class="text-red-700" />
        <carbon-badge v-if="version.stable" class="text-green-700" />
        <span>{{ version.stable ? 'stable' : 'unstable' }}</span>
      </div>
      <span class="flex-1">{{ version.version }}</span>
      <CustomButton small short green @click="install(version.version)">
        Install
      </CustomButton>
    </li>
  </ul>
  <div class="flex gap-x-2 justify-end w-full">
    <span>Show unstable versions</span>
    <Switch v-model="showUnstable" :class="showUnstable ? 'bg-teal-900' : 'bg-teal-700'" class="relative inline-flex items-center h-6 rounded-full w-11 cursor-pointer focus:outline-none">
      <span class="sr-only">Enable snapshots</span>
      <span
        :class="showUnstable ? 'translate-x-6' : 'translate-x-1'"
        class="inline-block w-4 h-4 transform bg-white rounded-full"
      />
    </Switch>
  </div>
</template>
