<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/tauri'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const router = useRouter()

const email = ref<string>()
const password = ref<string>()

const addAccount = () => {
  invoke('login', {
    email: email.value,
    password: password.value,
  })
    .then(() => router.push('/accounts'))
    .catch((e: string) => console.error(e))
}
</script>

<template>
  <div class="flex flex-col gap-y-4 my-auto min-w-64">
    <h1 class="text-3xl text-center my-4">
      {{ t('accounts.add') }}
    </h1>
    <div class="flex flex-col gap-y-4">
      <label class="flex flex-col">
        <span>{{ t('accounts.email') }}</span>
        <input v-model="email" type="email" class="rounded-lg border-gray-300 shadow-md dark:bg-black" />
      </label>
      <label class="flex flex-col">
        <span>{{ t('accounts.password') }}</span>
        <input v-model="password" type="password" class="rounded-lg border-gray-300 shadow-md dark:bg-black" />
      </label>
      <button class="btn bg-primary-500" @click="addAccount">
        {{ t('add') }}
      </button>
    </div>
  </div>
</template>
