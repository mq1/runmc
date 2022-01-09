<script setup lang="ts">
import { list, invalidate } from '~/logic/accounts'
import type { Account } from '~/logic/accounts'

const { t } = useI18n()

const availableAccounts = ref<Account[]>()
const updateAvailableAccounts = async() =>
  availableAccounts.value = await list()

const removeAccount = (account: Account) =>
  invalidate(account)
    .then(updateAvailableAccounts)

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
