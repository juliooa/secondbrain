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
					notice = "You don't have any language model set. Go to Config to set one.";
				} else {
					goto('/qa');
				}
			}
		})
		.catch((error) => console.error(error));
</script>

<div class="text-center h-screen">
	{#if notice != ''}
		<h2 class=" font-bold tracking-tight">{notice}</h2>
		<button type="button" class="btn variant-filled" on:click={() => goto('/config')}
			>Go to configuration</button
		>
	{/if}
</div>
