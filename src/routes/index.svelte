<script lang="ts">
	import { onMount } from 'svelte';
	import { getInstalledVersions, executeVersion } from '$lib/api';
	import type { Version } from '$lib/api';

	let versions: Version[] = [];
	const updateVersions = () => {
		getInstalledVersions().then((v) => {
			versions = v;
		});
	};

	let isVersionSelectorDropdownOpen = false;
	const toggleDropdown = () => {
		isVersionSelectorDropdownOpen = !isVersionSelectorDropdownOpen;
	};

	let selectedVersion: Version = { id: 'Select version ↓', url: '' };
	const selectVersion = (version: Version) => {
		selectedVersion = version;
		isVersionSelectorDropdownOpen = false;
	};

	let accessToken = '';

	onMount(updateVersions);
</script>

<h1 class="text-5xl">runmc</h1>

<div class="my-8 flex">
	<button class="border-2 w-40 rounded-l-full py-2 px-4" on:click={toggleDropdown}>
		{selectedVersion.id}
	</button>
	{#if isVersionSelectorDropdownOpen}
		<div
			class="absolute border-2 rounded-3xl mt-12 w-40 flex flex-col items-center divide-y bg-white"
		>
			{#each versions as version}
				<button class="p-2 w-full text-center" on:click={(e) => selectVersion(version)}>
					{version.id}
				</button>
			{/each}
			<a class="p-2 w-full text-center" href="/">+</a>
		</div>
	{/if}
	<button
		class="text-white bg-blue-500 border-2 border-blue-500 rounded-r-full py-2 px-4"
		on:click={(e) => executeVersion(selectedVersion, accessToken)}>▶</button
	>
</div>

<div>
	Access token
	<input class="border-2 rounded-full p-2" v-model="accessToken" />
</div>
