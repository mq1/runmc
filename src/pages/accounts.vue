<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { useI18n } from 'vue-i18n'
import type { Account } from '~/types'

const { t } = useI18n()

const availableAccounts = ref<Account[]>()
const updateAvailableAccounts = () => {
  invoke('get_accounts')
    .then((a) => {
      availableAccounts.value = a as Account[]
    })
    .catch((e: string) => console.error(e))
}

const removeAccount = (account: Account) => {
  invoke('remove_account', { account })
    .then(updateAvailableAccounts)
    .catch((e: string) => console.error(e))
}

onMounted(updateAvailableAccounts)
</script>

<template>
  <h1 class="text-3xl text-center my-4">
    {{ t('accounts.available') }}
  </h1>

  <div class="min-w-xs flex flex-col gap-y-4">
    <div
      v-for="account in availableAccounts"
      :key="account.id"
      class="box border flex justify-between items-center"
    >
      {{ account.name }}
      <button class="tiny bg-red-500 text-white" @click="removeAccount(account)">
        <carbon-trash-can />
      </button>
    </div>

    <router-link to="/add-account">
      <button class="primary">
        {{ t('accounts.add') }}
      </button>
    </router-link>
  </div>

  <div />
</template>
