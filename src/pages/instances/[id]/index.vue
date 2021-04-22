<script setup lang="ts">
import { defineProps } from 'vue'

const props = defineProps({
  id: {
    type: String,
    required: true,
  },
})
</script>

<template>
  <div class="flex flex-col gap-y-4 w-full">
    <h1 class="text-3xl text-center my-4">
      {{ props.id }}
    </h1>
    <h2 class="text-2xl">
      Mod loaders
    </h2>
    <div class="flex justify-between items-center">
      <div>Fabric</div>
      <div>not installed (TODO implement)</div>
      <router-link :to="`${props.id}/install-fabric`">
        <CustomButton green small>
          install
        </CustomButton>
      </router-link>
    </div>
    <h2 class="text-2xl">
      Mods
    </h2>
  </div>
</template>
