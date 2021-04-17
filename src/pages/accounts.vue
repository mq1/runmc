<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const email = ref('')
const password = ref('')

const availableAccounts = ref<any[]>([])
const updateAvailableAccounts = () => {
  invoke('accounts')
    .then(a => availableAccounts.value = a as any[])
    .catch((e: string) => console.error(e))
}

const addAccount = () => {
  invoke('login', {
    email: email.value,
    password: password.value,
  })
    .then(updateAvailableAccounts)
    .catch((e: string) => console.error(e))
}

onMounted(updateAvailableAccounts)
</script>

<template>
  <div class="flex justify-between gap-16">
    <div class="flex flex-col gap-8 w-72">
      <h1 class="text-3xl text-center">
        Add account
      </h1>
      <div class="flex flex-col gap-4">
        email: <input v-model="email" type="email" />
        password: <input v-model="password" type="password" />
        <button class="rounded-full py-2 px-4 bg-blue-500 text-white" @click="addAccount">
          Add
        </button>
      </div>
    </div>
    <div class="flex flex-col gap-8 w-72">
      <h1 class="text-3xl text-center">
        Available accounts
      </h1>
      <div class="border-2 rounded-3xl p-2 overflow-y-auto h-64 flex flex-col divide-y">
        <button v-for="account in availableAccounts" :key="account.id" class="p-2">
          {{ account.name }}
        </button>
      </div>
    </div>
  </div>
</template>
