<script setup lang="ts">
import { defineProps, onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import type { InstanceInfo } from '~/types'

const props = defineProps({
  id: {
    type: String,
    required: true,
  },
})

const instance = ref<InstanceInfo>()
const updateInstance = () => {
  invoke('get_instance_info', {
    instanceName: props.id,
  })
    .then(i => instance.value = i as InstanceInfo)
    .catch((e: string) => console.error(e))
}

const mods = ref<string[]>()
const updateMods = () => {
  invoke('list_mods', {
    instanceName: props.id,
  })
    .then(m => mods.value = m as string[])
    .catch((e: string) => console.error(e))
}
const openModsDir = () => {
  invoke('open_mods_dir', {
    instanceName: props.id,
  })
    .catch((e: string) => console.error(e))
}

onMounted(() => {
  updateInstance()
  updateMods()
})
</script>

<template>
  <div class="flex flex-col gap-y-6 w-full">
    <h1 class="text-3xl text-center my-4">
      Instance {{ props.id }}
    </h1>
    <h2 class="text-2xl">
      Mod loaders
    </h2>
    <div class="flex justify-between items-center">
      <div>Fabric</div>
      <div v-if="!instance?.fabric" class="text-red-700">
        not installed
      </div>
      <div v-if="instance?.fabric" class="text-green-700">
        installed
      </div>
      <router-link :to="`${props.id}/install-fabric`">
        <CustomButton green small>
          install
        </CustomButton>
      </router-link>
    </div>
    <div class="flex justify-between">
      <h2 class="text-2xl">
        Mods
      </h2>
      <CustomButton blue small short @click="openModsDir">
        Open mods directory
      </CustomButton>
    </div>
    <ul>
      <li v-for="mod in mods" :key="mod">
        {{ mod }}
      </li>
    </ul>
  </div>
</template>
