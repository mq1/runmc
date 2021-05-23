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
  <h1 class="text-3xl text-center my-4">
    {{ t('instances.available') }}
  </h1>

  <div class="min-w-xs flex flex-col gap-y-4">
    <Instance
      v-for="instance in instances"
      :id="instance"
      :key="instance"
      @update="updateInstances"
    />

    <router-link to="/new-instance">
      <button class="primary">
        {{ t('instances.new') }}
      </button>
    </router-link>
  </div>

  <div />
</template>
