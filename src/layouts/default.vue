<template>
  <div class="flex flex justify-between h-screen dark:bg-black dark:text-white">
    <nav class="flex flex-col py-2 w-36 bg-purple-500 justify-between">
      <div class="flex flex-col gap-y-2">
        <LinkButton to="/">
          <heroicons-outline-home /> Home
        </LinkButton>
        <LinkButton to="/accounts">
          <heroicons-outline-users /> Accounts
        </LinkButton>
        <LinkButton to="/versions">
          <heroicons-outline-collection /> Versions
        </LinkButton>
      </div>
      <div>
        <LinkButton to="/info">
          <heroicons-outline-information-circle /> Info
        </LinkButton>
      </div>
    </nav>
    <main class="flex-1 flex flex-col items-center justify-center">
      <router-view />
    </main>
  </div>
</template>
