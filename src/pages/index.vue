<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { useI18n } from 'vue-i18n'
import type { Account } from '~/types'

const { t } = useI18n()

const accounts = ref<Account[]>()
const selectedAccount = ref<Account>({ name: 'No users found', id: '', accessToken: '' })

const updateAccounts = () => {
  invoke('list_accounts')
    .then((a) => {
      accounts.value = a as Account[]
      selectedAccount.value = accounts.value[0] || { name: 'No users found', id: '', accessToken: '' }
    })
    .catch((e: string) => console.error(e))
}

const instances = ref<string[]>()
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
    instanceName: selectedInstance.value,
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
  <div class="flex flex-col items-center justify-center gap-y-4 h-full">
    <div class="text-3xl text-gray-500">
      {{ t('index.welcome') }}
    </div>

    <select v-model="selectedAccount" class="min-w-80 min-h-24 rounded-2xl text-5xl">
      <option v-for="account in accounts" :key="account.id" :value="account">
        {{ account.name }}
      </option>
    </select>

    <div class="flex gap-x-4">
      <select v-model="selectedInstance" class="min-w-40">
        <option v-for="instance in instances" :key="instance">
          {{ instance }}
        </option>
      </select>

      <button class="primary round" @click="runInstance">
        <carbon-arrow-right />
      </button>
    </div>
  </div>
</template>
