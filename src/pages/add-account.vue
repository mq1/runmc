<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

const email = ref('')
const password = ref('')

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
  <div class="flex flex-col gap-8">
    <h1 class="text-3xl text-center">
      Add an account
    </h1>
    <div class="flex flex-col gap-y-4">
      <label class="flex flex-col">
        <span>Email</span>
        <input v-model="email" type="email" class="rounded-lg border-gray-300 shadow-md dark:bg-black" />
      </label>
      <label class="flex flex-col">
        <span>Password</span>
        <input v-model="password" type="password" class="rounded-lg border-gray-300 shadow-md dark:bg-black" />
      </label>
      <button class="rounded-lg shadow-md py-2 px-4 bg-purple-500 text-white" @click="addAccount">
        Add
      </button>
    </div>
  </div>
</template>
