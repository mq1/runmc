<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const instances = ref<string[]>()
const updateInstances = () => {
  invoke('list_instances')
    .then(i => instances.value = i as string[])
    .catch((e: string) => console.error(e))
}

onMounted(updateInstances)
</script>

<template>
  <div class="flex flex-col gap-y-4 my-auto min-w-64">
    <h1 class="text-3xl text-center my-4">
      {{ t('instances.available') }}
    </h1>
    <div
      v-for="instance in instances"
      :key="instance"
      class="box flex justify-between items-center"
    >
      <Instance :id="instance" @update="updateInstances" />
    </div>
    <router-link to="/new-instance" class="btn bg-primary-500">
      {{ t('instances.new') }}
    </router-link>
  </div>
</template>
