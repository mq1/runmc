<script setup lang="ts">
import { ref, defineProps, defineEmit } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/tauri'

const props = defineProps({
  version: {
    type: String,
    required: true,
  },
})

const emit = defineEmit(['update'])

const router = useRouter()

const renaming = ref(false)
const name = ref(props.version)

const startRenaming = () => {
  renaming.value = true
}

const renameVersion = () => {
  if (props.version === name.value) {
    renaming.value = false
    return
  }

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
    <SettingsButton @click="router.push(`/versions/${props.version}`)" />
    <RemoveButton @click="removeVersion" />
  </div>
</template>
