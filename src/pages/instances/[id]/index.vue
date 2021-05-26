<script setup lang="ts">
import { defineProps, onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { useI18n } from 'vue-i18n'
import type { InstanceInfo } from '~/types'

const { t } = useI18n()

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
  <h1 class="text-3xl text-center my-4">
    {{ t('instances.instance', { id: props.id }) }}
  </h1>

  <div class="min-w-full flex flex-col gap-y-8">
    <div class="box border">
      <h2 class="text-2xl">
        {{ t('instances.modloaders') }}
      </h2>

      <br />

      <div class="flex justify-between items-center">
        <div>Fabric</div>
        <div v-if="instance?.fabric" class="text-green-700">
          {{ t('installed') }}
        </div>
        <router-link v-if="!instance?.fabric" :to="`${props.id}/install-fabric`">
          <button class="small bg-green-500 text-white">
            {{ t('install') }}
          </button>
        </router-link>
      </div>
    </div>

    <div class="box border">
      <h2 class="text-2xl">
        {{ t('instances.mods') }}
      </h2>

      <br />

      <ul class="list-disc ml-6">
        <li v-for="mod in mods" :key="mod">
          {{ mod }}
        </li>
      </ul>

      <br />

      <div class="w-full flex justify-end">
        <button class="small bg-blue-500 text-white w-auto" @click="openModsDir">
          {{ t('instances.openmodsdir') }}
        </button>
      </div>
    </div>
  </div>

  <div />
</template>
