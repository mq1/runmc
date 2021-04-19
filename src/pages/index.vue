<script setup lang="ts">
import { Listbox, ListboxButton, ListboxOptions, ListboxOption } from '@headlessui/vue'
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import type { Account } from '~/types'

const availableAccounts = ref<Account[]>([])
const selectedAccount = ref<Account>({ name: '', id: '', access_token: '' })

const updateAvailableAccounts = () => {
  invoke('accounts')
    .then((a) => {
      availableAccounts.value = a as Account[]
      selectedAccount.value = availableAccounts.value[0] || { name: 'No users found', id: '', access_token: '' }
    })
    .catch((e: string) => console.error(e))
}

const versions = ref<string[]>([])
const selectedVersion = ref('Select version')

const getVersions = () => {
  invoke('list_versions')
    .then((v) => {
      versions.value = v as string[]
      selectedVersion.value = versions.value[0] || 'No versions installed'
    })
    .catch((e: string) => console.error(e))
}

const executeVersion = () => {
  invoke('run_minecraft', {
    version: selectedVersion.value,
    account: selectedAccount.value,
  })
    .catch((e: string) => console.error(e))
}

onMounted(() => {
  getVersions()
  updateAvailableAccounts()
})
</script>

<template>
  <div class="text-3xl text-gray-500">
    welcome back,
  </div>
  <Listbox v-model="selectedAccount">
    <ListboxButton class="my-4 border-1 px-4 py-2 rounded-lg shadow-md flex justify-between text-5xl min-w-72 focus:outline-none cursor-default">
      {{ selectedAccount.name }}
      <heroicons-outline-selector class="text-gray-400" />
    </ListboxButton>
    <ListboxOptions class="absolute mt-36 bg-white dark:bg-black border-1 rounded-lg border-1 shadow-md w-max min-w-72 list-none flex flex-col divide-y focus:outline-none">
      <ListboxOption v-for="account in availableAccounts" v-slot="{ selected }" :key="account.id" :value="account">
        <div class="p-2">
          <span v-if="selected" class="absolute inset-y-0 left-0 flex items-center pl-3"><heroicons-outline-check /></span>
          {{ account.name }}
        </div>
      </ListboxOption>
    </ListboxOptions>
  </Listbox>

  <div class="flex">
    <Listbox v-model="selectedVersion">
      <ListboxButton class="border-1 rounded-lg shadow-md py-2 px-4 w-40 flex items-center justify-between cursor-default focus:outline-none">
        {{ selectedVersion }}
        <heroicons-outline-selector class="text-gray-400" />
      </ListboxButton>
      <ListboxOptions class="absolute bg-white dark:bg-black mt-12 w-40 flex flex-col divide-y border-1 rounded-lg shadow-md list-none focus:outline-none">
        <ListboxOption v-for="version in versions" v-slot="{ selected }" :key="version" :value="version">
          <div class="p-2">
            <span v-if="selected" class="absolute inset-y-0 left-0 flex items-center pl-3"><heroicons-outline-check /></span>
            {{ version }}
          </div>
        </ListboxOption>
      </ListboxOptions>
    </Listbox>
    <button
      class="text-white bg-purple-500 rounded-full shadow-md p-3 mx-4 flex items-center focus:outline-none"
      @click="executeVersion"
    >
      <heroicons-outline-arrow-right />
    </button>
  </div>
</template>
