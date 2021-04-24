<template>
  <div class="flex flex justify-between h-screen dark:bg-black dark:text-white">
    <nav class="flex flex-col p-2 w-40 bg-purple-500 dark:bg-purple-700 justify-between">
      <div class="flex flex-col gap-y-2">
        <router-link v-slot="{ isExactActive }" to="/">
          <CustomButton small :selected="isExactActive">
            <carbon-home /> Home
          </CustomButton>
        </router-link>
        <router-link v-slot="{ isExactActive }" to="/accounts">
          <CustomButton small :selected="isExactActive">
            <carbon-user-multiple /> Accounts
          </CustomButton>
        </router-link>
        <router-link v-slot="{ isExactActive }" to="/instances">
          <CustomButton small :selected="isExactActive">
            <carbon-layers /> Instances
          </CustomButton>
        </router-link>
      </div>
      <div class="flex flex-col gap-y-2">
        <router-link v-slot="{ isExactActive }" to="/settings">
          <CustomButton small :selected="isExactActive">
            <carbon-settings-adjust /> Settings
          </CustomButton>
        </router-link>
        <router-link v-slot="{ isExactActive }" to="/info">
          <CustomButton small :selected="isExactActive">
            <carbon-information /> Info
          </CustomButton>
        </router-link>
      </div>
    </nav>
    <main class="flex-1 flex flex-col items-center justify-between gap-y-8 m-8 max-h-screen overflow-auto">
      <router-view />
    </main>
  </div>
</template>
