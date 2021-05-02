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
      <ListboxButton class="box px-6 py-4 text-5xl my-4 flex items-center justify-between min-w-80 text-left">
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
        <ListboxOptions class="absolute mt-38 min-w-80 box">
          <ListboxOption
            v-for="account in accounts"
            v-slot="{ selected }"
            :key="account.id"
            :value="account"
            as="div"
            class="p-2 w-full"
          >
            <span v-if="selected" class="absolute inset-y-0 left-0 flex items-center pl-3"><carbon-checkmark /></span>
            <span class="pl-8">{{ account.name }}</span>
          </ListboxOption>
        </ListboxOptions>
      </transition>
    </Listbox>

    <div class="flex gap-x-2">
      <Listbox v-model="selectedInstance">
        <ListboxButton class="box min-w-40 small flex items-center justify-between">
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
          <ListboxOptions class="absolute mt-12 min-w-40 box">
            <ListboxOption
              v-for="instance in instances"
              v-slot="{ selected }"
              :key="instance"
              :value="instance"
              as="div"
              class="p-2 flex items-center w-full"
            >
              <span v-if="selected" class="absolute inset-y-0 left-0 flex items-center pl-3"><carbon-checkmark /></span>
              <span class="pl-8">{{ instance }}</span>
            </ListboxOption>
          </ListboxOptions>
        </transition>
      </Listbox>
      <button class="btn round bg-primary-500" @click="runInstance">
        <carbon-arrow-right />
      </button>
    </div>
  </div>
</template>
