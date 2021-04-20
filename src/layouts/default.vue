<template>
  <div class="flex flex justify-between h-screen dark:bg-black dark:text-white">
    <nav class="flex flex-col p-2 w-36 bg-purple-500 justify-between">
      <div class="flex flex-col gap-y-2">
        <CustomButton as="router-link" to="/" small>
          <heroicons-outline-home /> Home
        </CustomButton>
        <CustomButton as="router-link" to="/accounts" small>
          <heroicons-outline-users /> Accounts
        </CustomButton>
        <CustomButton as="router-link" to="/versions" small>
          <heroicons-outline-collection /> Versions
        </CustomButton>
      </div>
      <div class="flex flex-col gap-y-2">
        <CustomButton as="router-link" to="/settings" small>
          <heroicons-outline-cog /> Settings
        </CustomButton>
        <CustomButton as="router-link" to="/info" small>
          <heroicons-outline-information-circle /> Info
        </CustomButton>
      </div>
    </nav>
    <main class="flex-1 flex flex-col items-center justify-center max-h-screen overflow-auto">
      <router-view />
    </main>
  </div>
</template>
