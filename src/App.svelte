<script lang="ts">
  import Home from "./lib/Home.svelte";
  import Versions from "./lib/Versions.svelte";

  let selectedView: any = Home;
</script>

<div class="p-4 flex flex-col justify-between h-screen">
  <nav class="flex justify-between">
    <button on:click={(e) => (selectedView = Home)}>🏠</button>
  </nav>

  <main class="flex flex-col items-center justify-center">
    <svelte:component
      this={selectedView}
      on:goToVersions={(e) => (selectedView = Versions)}
    />
  </main>

  <footer class="mx-auto">MIT Licensed | © 2021 Manuel Quarneti</footer>
</div>
