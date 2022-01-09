<script setup lang="ts">
import { authenticate } from '~/logic/accounts'

const { t } = useI18n()
const router = useRouter()

const email = ref<string>('')
const password = ref<string>('')

const addAccount = () =>
  authenticate(email.value, password.value)
    .then(() => router.push('/accounts'))

</script>

<template>
  <div class="flex flex-col gap-y-4 my-auto min-w-64">
    <h1 class="text-3xl text-center my-4">
      {{ t('accounts.add') }}
    </h1>
    <div class="flex flex-col gap-y-4">
      <label class="flex flex-col">
        <span>{{ t('accounts.email') }}</span>
        <input v-model="email" type="email" />
      </label>
      <label class="flex flex-col">
        <span>{{ t('accounts.password') }}</span>
        <input v-model="password" type="password" />
      </label>
      <button class="primary" @click="addAccount">
        {{ t('add') }}
      </button>
    </div>
  </div>
</template>
