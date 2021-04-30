<script setup lang="ts">
import { Listbox, ListboxButton, ListboxOptions, ListboxOption } from '@headlessui/vue'
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import type { Account } from '~/types'

const accounts = ref<Account[]>([])
const selectedAccount = ref<Account>({ name: 'No users found', id: '', access_token: '' })

const updateAccounts = () => {
  invoke('get_accounts')
    .then((a) => {
      accounts.value = a as Account[]
      selectedAccount.value = accounts.value[0] || { name: 'No users found', id: '', access_token: '' }
    })
    .catch((e: string) => console.error(e))
}

const instances = ref<string[]>([])
const selectedInstance = ref('No instances present')

const updateInstances = () => {
  invoke('list_instances')
    .then((i) => {
      instances.value = i as string[]
      selectedInstance.value = instances.value[0] || 'No instances present'
    })
    .catch((e: string) => console.error(e))
}

const runInstance = () => {
  invoke('run_instance', {
    instance: selectedInstance.value,
    account: selectedAccount.value,
  })
    .catch((e: string) => console.error(e))
}

onMounted(() => {
  updateInstances()
  updateAccounts()
})
</script>

<template>
  <div class="flex flex-col items-center my-auto">
    <div class="text-3xl text-gray-500">
      welcome back,
    </div>
    <Listbox v-model="selectedAccount">
      <ListboxButton class="my-4 border-1 p-2 pl-4 rounded-lg shadow-md flex justify-between text-5xl min-w-80 focus:outline-none cursor-pointer">
        {{ selectedAccount.name }}
        <carbon-chevron-sort class="text-gray-400" />
      </ListboxButton>
      <transition
        enter-active-class="transition duration-100 ease-out"
        enter-from-class="transform scale-95 opacity-0"
        enter-to-class="transform scale-100 opacity-100"
        leave-active-class="transition duration-75 ease-in"
        leave-from-class="transform scale-100 opacity-100"
        leave-to-class="transform scale-95 opacity-0"
      >
        <ListboxOptions class="absolute mt-36 bg-white dark:bg-black border-1 rounded-lg border-1 shadow-md w-max min-w-72 list-none flex flex-col divide-y focus:outline-none cursor-pointer">
          <ListboxOption
            v-for="account in accounts"
            v-slot="{ selected }"
            :key="account.id"
            :value="account"
            as="div"
            class="p-2"
          >
            <span v-if="selected" class="absolute inset-y-0 left-0 flex items-center pl-3"><carbon-checkmark /></span>
            <span class="pl-8">{{ account.name }}</span>
          </ListboxOption>
        </ListboxOptions>
      </transition>
    </Listbox>

    <div class="flex">
      <Listbox v-model="selectedInstance">
        <ListboxButton class="border-1 rounded-lg shadow-md p-2 pl-4 min-w-40 flex items-center justify-between cursor-pointer focus:outline-none">
          {{ selectedInstance }}
          <carbon-chevron-sort class="text-gray-400" />
        </ListboxButton>
        <transition
          enter-active-class="transition duration-100 ease-out"
          enter-from-class="transform scale-95 opacity-0"
          enter-to-class="transform scale-100 opacity-100"
          leave-active-class="transition duration-75 ease-in"
          leave-from-class="transform scale-100 opacity-100"
          leave-to-class="transform scale-95 opacity-0"
        >
          <ListboxOptions class="absolute bg-white dark:bg-black mt-14 w-40 flex flex-col divide-y border-1 rounded-lg shadow-md list-none focus:outline-none cursor-pointer">
            <ListboxOption
              v-for="instance in instances"
              v-slot="{ selected }"
              :key="instance"
              :value="instance"
              as="div"
              class="p-2 flex items-center"
            >
              <span v-if="selected" class="absolute inset-y-0 left-0 flex items-center pl-3"><carbon-checkmark /></span>
              <span class="pl-8">{{ instance }}</span>
            </ListboxOption>
          </ListboxOptions>
        </transition>
      </Listbox>
      <Button class="bg-purple-500 rounded-full shadow-md p-3 text-white ml-3 h-min w-min flex justify-center items-center focus:outline-none" @click="runInstance">
        <carbon-arrow-right />
      </Button>
    </div>
  </div>
</template>
