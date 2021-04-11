<template>
  <div class="m-4">
    <nav class="flex justify-between">
      <div>
        <router-link to="/">🏠</router-link>
      </div>
      <div>
        <router-link to="/about">?</router-link>
      </div>
    </nav>
    <router-view />

    <footer>
      MIT Licensed | © 2021 Manuel Quarneti
    </footer>
  </div>
</template>
