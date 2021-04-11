<template>
  <div class="p-4 flex flex-col justify-between h-screen">
    <nav class="flex justify-between">
      <div>
        <router-link to="/">🏠</router-link>
      </div>
      <div>
        <router-link to="/about">?</router-link>
      </div>
    </nav>

    <main class="m-auto">
      <router-view />
    </main>

    <footer class="text-center min-h-max">
      MIT Licensed | © 2021 Manuel Quarneti
    </footer>
  </div>
</template>
