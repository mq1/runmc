<script setup lang="ts">
import { listInstances } from '~/logic/instances'

const { t } = useI18n()

const instances = ref<string[]>([])
const updateInstances = async() =>
  instances.value = await listInstances()

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
