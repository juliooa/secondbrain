<script lang="ts">
	import { open as shellOpen } from '@tauri-apps/api/shell';
	import type { PageData } from './$types';
	import { goto } from '$app/navigation';

	export let data: PageData;

	function openRepo() {
		shellOpen('https://github.com/juliooa/secondbrain');
	}
</script>

<div class="flex flex-col justify-center items-center h-full">
	<h1 class="gradient-heading h-16">Welcome to your Second Brain</h1>
	<div class="mt-4 space-y-4">
		<h4>🚀 Unleash the Power of AI</h4>
		<h4>💻 In your device - it can work without internet</h4>
		<h4>🔒 Privacy first - your messages doesn't leave your computer.</h4>
		<h4>
			📖 Open source - <a href="/" on:keydown={() => openRepo()} on:click={() => openRepo()}
				>check here</a
			>
		</h4>
	</div>
	{#if data.activeModel == null}
		<div class="card m-6 p-4 flex flex-col items-center">
			<h4 class="font-bold">You don't have any language model set. Go to config to set one</h4>
			<button type="button" class="btn variant-filled mt-4" on:click={() => goto('/config')}
				>Go to configuration</button
			>
		</div>
	{/if}
</div>
