<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const availableAccounts = ref<{ name: string; id: string; access_token: string }[]>([])
const selectedAccount = ref({ name: '', id: '', access_token: '' })

const updateAvailableAccounts = () => {
  invoke('accounts')
    .then((a) => {
      availableAccounts.value = a as any[]
      selectedAccount.value = availableAccounts.value[0] || { name: '', id: '', access_token: '' }
    })
    .catch((e: string) => console.error(e))
}

const accountDropdown = ref(false)
const toggleAccountDropdown = () => {
  accountDropdown.value = !accountDropdown.value
}

const selectAccount = (account: any) => {
  selectedAccount.value = account
  accountDropdown.value = false
}

onMounted(updateAvailableAccounts)
</script>

<template>
  <div class="p-4 flex flex-col justify-between h-screen">
    <nav class="flex justify-between">
      <router-link to="/">
        🏠
      </router-link>
      <router-link v-if="availableAccounts.length === 0" to="/accounts">
        Add account
      </router-link>
      <div v-if="availableAccounts.length > 0">
        <button @click="toggleAccountDropdown">
          {{ selectedAccount.name }}
        </button>
        <div v-show="accountDropdown" class="absolute mt-2 flex flex-col divide-y text-center">
          <button v-for="account in availableAccounts" :key="account.id" class="py-2" @click="selectAccount(account)">
            {{ account.name }}
          </button>
          <router-link class="py-2" to="/accounts" @click="toggleAccountDropdown">
            +
          </router-link>
        </div>
      </div>
    </nav>

    <main class="flex flex-col items-center justify-center">
      <router-view :access-token="selectedAccount.access_token" @accountsUpdate="updateAvailableAccounts" />
    </main>

    <footer class="mx-auto">
      MIT Licensed | © 2021 Manuel Quarneti
    </footer>
  </div>
</template>
