<script lang="ts">
	import { onMount } from 'svelte';
	import {
		getAvailableVersions,
		getInstalledVersions,
		installVersion,
		removeVersion
	} from '$lib/api';
	import type { Version } from '$lib/api';

	let installedVersions: Version[] = [];
	const updateInstalledVersions = () => {
		getInstalledVersions().then((versions) => {
			installedVersions = versions;
		});
	};

	let availableVersions: Version[] = [];
	const updateAvailableVersions = () => {
		getAvailableVersions().then((versions) => {
			availableVersions = versions;
		});
	};

	const install = (version: Version) => {
		installVersion(version).then(updateInstalledVersions);
	};

	const remove = (version: Version) => {
		removeVersion(version).then(updateInstalledVersions);
	};

	onMount(() => {
		updateInstalledVersions();
		updateAvailableVersions();
	});
</script>

<div class="flex gap-x-8">
	{#if installedVersions.length}
		<div class="flex-1">
			<h1 class="text-3xl text-center mb-8">Installed Versions</h1>
			<div class="border-2 rounded-3xl p-2 overflow-y-auto h-64 flex flex-col divide-y">
				{#each installedVersions as version}
					<div class="p-2 flex justify-between">
						{version.id}
						<button
							class="px-2 py-1 bg-red-500 text-white rounded-full"
							on:click={(e) => remove(version)}>Remove 🗑</button
						>
					</div>
				{/each}
			</div>
		</div>
	{/if}
	<div class="flex-1">
		<h1 class="text-3xl text-center mb-8">Available Versions</h1>
		<div class="border-2 rounded-3xl p-2 overflow-y-auto h-64 flex flex-col divide-y">
			{#each availableVersions as version}
				<div class="p-2 flex justify-between">
					{version.type === 'release' ? '✅' : '🔥'}
					{version.type}
					{version.id}
					<button
						class="px-2 py-1 bg-green-500 text-white rounded-full"
						on:click={(e) => install(version)}>Install ➕</button
					>
				</div>
			{/each}
		</div>
	</div>
</div>
