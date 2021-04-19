<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import type { Account } from '~/types'

const availableAccounts = ref<Account[]>([])
const updateAvailableAccounts = () => {
  invoke('accounts')
    .then((a) => {
      availableAccounts.value = a as Account[]
    })
    .catch((e: string) => console.error(e))
}

const removeAccount = (account: Account) => {
  invoke('remove_account', {
    name: account.name,
  })
    .then(updateAvailableAccounts)
    .catch((e: string) => console.error(e))
}

onMounted(updateAvailableAccounts)
</script>

<template>
  <div class="flex flex-col gap-y-4">
    <h1 class="text-3xl text-center my-4">
      Available accounts
    </h1>
    <div v-for="account in availableAccounts" :key="account.id" class="flex p-2 border-1 rounded-lg shadow-md flex justify-between">
      {{ account.name }}
      <RemoveButton @click="removeAccount(account)" />
    </div>
    <CustomButton as="router-link" to="/add-account" color="purple" center>
      Add an account
    </CustomButton>
  </div>
</template>
