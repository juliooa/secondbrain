<script lang="ts">
	import { goto } from '$app/navigation';
	import type { CommandResponseLanguagesModels, LanguageModel } from '$lib/types';
	import { invoke } from '@tauri-apps/api';

	let notice: string = '';
	invoke<CommandResponseLanguagesModels>('get_language_models')
		.then((languageModelsResponse) => {
			let languageModels: LanguageModel[] = languageModelsResponse.models;
			if (languageModels.length == 0) {
				notice = "You don't have any language model downloaded. Go to Config to download one.";
			} else {
				if (languageModels.filter((model) => model.current == true).length == 0) {
					notice = "You don't have any language model set. Go to config to set one.";
				} else {
					//goto('/qa');
				}
			}
		})
		.catch((error) => console.error(error));
</script>

<div class="h-screen flex flex-col justify-center items-center">
	<h1 class="gradient-heading h-16">Welcome to your Second Brain</h1>
	<div class="mt-4 space-y-4">
		<h4>🚀 Unleash the Power of AI</h4>
		<h4>💻 In your device - it can work without internet</h4>
		<h4>🔒 Privacy first - your messages doesn't leave your computer.</h4>
		<h4>📖 Open source - <a href="/">check here</a></h4>
	</div>
	{#if notice != ''}
		<div class="card m-6 p-4 flex flex-col items-center">
			<h4 class="font-bold">{notice}</h4>
			<button type="button" class="btn variant-filled mt-4" on:click={() => goto('/config')}
				>Go to configuration</button
			>
		</div>
	{/if}
</div>

<!-- <div class="h-screen flex flex-col justify-center items-center">
	<div class="p-4">
		<h2>Query your documents</h2>
		<div class="flex items-center">
			<p class="text-xl text-error-500">No active model</p>
		</div>
	</div>
	<h3 class="mt-3">coming soon... 🚀</h3>
</div> -->
