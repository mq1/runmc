<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'

const props = defineProps({
  id: {
    type: String,
    required: true,
  },
})

const emit = defineEmits(['update'])

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

  invoke('rename_instance', { oldName: props.id, newName: name.value })
    .then(() => emit('update'))
    .catch((e: string) => console.error(e))
}

const remove = () =>
  invoke('remove_instance', { name: props.id })
    .then(() => emit('update'))
    .catch((e: string) => console.error(e))
</script>

<template>
  <div class="box border flex justify-between items-center">
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
      <button v-if="!renaming" class="tiny bg-blue-500 text-white" @click="startRenaming">
        <carbon-pen />
      </button>
      <button v-if="renaming" class="tiny bg-green-500 text-white" @click="rename">
        <carbon-checkmark />
      </button>
      <button class="tiny bg-yellow-500 text-white" @click="router.push(`/instances/${props.id}`)">
        <carbon-settings />
      </button>
      <button class="tiny bg-red-500 text-white" @click="remove">
        <carbon-trash-can />
      </button>
    </div>
  </div>
</template>
