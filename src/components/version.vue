<script setup lang="ts">
import { ref, defineProps, defineEmit } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const props = defineProps({
  version: {
    type: String,
    required: true,
  },
})

const emit = defineEmit(['update'])

const renaming = ref(false)
const name = ref(props.version)

const startRenaming = () => {
  renaming.value = true
}

const renameVersion = () => {
  invoke('rename_version', { version: props.version, name: name.value })
    .then(() => emit('update'))
    .catch((e: string) => console.error(e))
}

const removeVersion = () => {
  invoke('remove_version', { version: props.version })
    .then(() => emit('update'))
    .catch((e: string) => console.error(e))
}
</script>

<template>
  <div v-if="!renaming">
    {{ props.version }}
  </div>
  <input v-if="renaming" v-model="name" type="text" class="border-gray-300 rounded-lg mr-2 dark:bg-black">
  <div class="flex gap-x-2">
    <RenameButton v-if="!renaming" @click="startRenaming" />
    <ApplyButton v-if="renaming" @click="renameVersion" />
    <RemoveButton @click="removeVersion" />
  </div>
</template>
