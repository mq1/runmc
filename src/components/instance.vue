<script setup lang="ts">
import { ref, defineProps, defineEmit } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/tauri'

const props = defineProps({
  id: {
    type: String,
    required: true,
  },
})

const emit = defineEmit(['update'])

const router = useRouter()

const renaming = ref(false)
const name = ref(props.id)

const startRenaming = () => {
  renaming.value = true
}

const rename = () => {
  if (props.id === name.value) {
    renaming.value = false
    return
  }

  invoke('rename_instance', {
    currentName: props.id,
    newName: name.value,
  })
    .then(() => emit('update'))
    .catch((e: string) => console.error(e))
}

const remove = () => {
  invoke('remove_instance', {
    version: props.id,
  })
    .then(() => emit('update'))
    .catch((e: string) => console.error(e))
}
</script>

<template>
  <div v-if="!renaming">
    {{ props.id }}
  </div>
  <input
    v-if="renaming"
    v-model="name"
    type="text"
    class="border-gray-300 rounded-lg mr-2 dark:bg-black"
  />
  <div class="flex gap-x-2">
    <RenameButton v-if="!renaming" @click="startRenaming" />
    <ApplyButton v-if="renaming" @click="rename" />
    <SettingsButton @click="router.push(`/instances/${props.id}`)" />
    <RemoveButton @click="remove" />
  </div>
</template>
