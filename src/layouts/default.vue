<template>
  <div class="p-4 flex flex-col justify-between h-screen dark:bg-black dark:text-white">
    <nav class="flex justify-between">
      <router-link class="flex items-center gap-x-2 py-1 px-2 border-2 rounded-full" to="/">
        <heroicons-outline-home /> Home
      </router-link>
      <div class="flex gap-x-4">
        <router-link class="flex items-center gap-x-2 py-1 px-2 border-2 rounded-full" to="/accounts">
          <heroicons-outline-users /> Accounts
        </router-link>
        <router-link class="flex items-center gap-x-2 py-1 px-2 border-2 rounded-full" to="/versions">
          <heroicons-outline-collection /> Versions
        </router-link>
      </div>
    </nav>

    <main class="flex flex-col items-center justify-center">
      <router-view />
    </main>

    <footer class="mx-auto">
      MIT Licensed | © 2021 Manuel Quarneti
    </footer>
  </div>
</template>
