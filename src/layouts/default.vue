<template>
  <div class="flex flex justify-between h-screen dark:bg-black dark:text-white">
    <nav class="flex flex-col p-2 w-40 bg-purple-500 dark:bg-purple-700 justify-between">
      <div class="flex flex-col gap-y-2">
        <router-link v-slot="{ isExactActive }" to="/">
          <CustomButton small :selected="isExactActive">
            <heroicons-outline-home /> Home
          </CustomButton>
        </router-link>
        <router-link v-slot="{ isExactActive }" to="/accounts">
          <CustomButton small :selected="isExactActive">
            <heroicons-outline-users /> Accounts
          </CustomButton>
        </router-link>
        <router-link v-slot="{ isExactActive }" to="/instances">
          <CustomButton small :selected="isExactActive">
            <heroicons-outline-collection /> Instances
          </CustomButton>
        </router-link>
      </div>
      <div class="flex flex-col gap-y-2">
        <router-link v-slot="{ isExactActive }" to="/settings">
          <CustomButton small :selected="isExactActive">
            <heroicons-outline-adjustments /> Settings
          </CustomButton>
        </router-link>
        <router-link v-slot="{ isExactActive }" to="/info">
          <CustomButton small :selected="isExactActive">
            <heroicons-outline-information-circle /> Info
          </CustomButton>
        </router-link>
      </div>
    </nav>
    <main class="flex-1 flex flex-col items-center justify-between gap-y-8 m-8 max-h-screen overflow-auto">
      <router-view />
    </main>
  </div>
</template>
