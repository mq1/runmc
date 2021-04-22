<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const instances = ref<string[]>([])
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
      Available instances
    </h1>
    <div
      v-for="instance in instances"
      :key="instance"
      class="p-2 flex items-center justify-between border-1 rounded-lg shadow-md"
    >
      <Instance :id="instance" @update="updateInstances" />
    </div>
    <router-link to="/new-instance">
      <CustomButton center primary>
        New instance
      </CustomButton>
    </router-link>
  </div>
</template>
