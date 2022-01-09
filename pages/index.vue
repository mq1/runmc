<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { list } from '~/logic/accounts'
import type { Account } from '~/logic/accounts'

const { t } = useI18n()

const accounts = ref<Account[]>()
const selectedAccount = ref<Account>({ name: 'No users found', id: '', accessToken: '' })

const updateAccounts = async() =>
  accounts.value = await list()

const instances = ref<string[]>([])
const selectedInstance = ref('')

const updateSelectedInstance = () =>
  invoke('read_config')
    .then(data => selectedInstance.value = (data as any).lastRunnedInstance)
    .catch(e => console.error(e))

const updateInstances = () =>
  invoke('get_instance_list')
    .then(data => instances.value = data as string[])
    .catch(e => console.error(e))

const runInstance = () => {
  invoke('run_instance', {
    instanceName: selectedInstance.value,
    account: selectedAccount.value,
  })
    .catch((e: string) => console.error(e))
}

onMounted(() => {
  updateSelectedInstance()
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
