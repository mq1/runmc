<script lang="ts">
	import 'virtual:windi.css';
</script>

<div class="p-4 flex flex-col justify-between h-screen">
	<nav class="flex justify-between">
		<a href="/versions">🏠</a>
	</nav>

	<main class="flex flex-col items-center justify-center">
		<slot />
	</main>

	<footer class="mx-auto">MIT Licensed | © 2021 Manuel Quarneti</footer>
</div>
