<template>
  <div class="p-4 flex flex-col justify-between h-screen">
    <nav class="flex justify-between">
      <router-link to="/">
        🏠
      </router-link>
    </nav>

    <main class="flex flex-col items-center justify-center">
      <router-view />
    </main>

    <footer class="mx-auto">
      MIT Licensed | © 2021 Manuel Quarneti
    </footer>
  </div>
</template>
