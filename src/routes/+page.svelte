<script lang="ts">
	import { goto } from '$app/navigation';
	import type { CommandResponseLanguagesModels, LanguageModel } from '$lib/types';
	import { invoke } from '@tauri-apps/api';

	let notice: string = '';
	//get models and default model
	invoke<CommandResponseLanguagesModels>('get_language_models')
		.then((languageModelsResponse) => {
			let languageModels: LanguageModel[] = languageModelsResponse.models;
			if (languageModels.length == 0) {
				notice = "You don't have any language model downloaded. Go to Config to download one.";
			} else {
				if (languageModels.filter((model) => model.default == true).length == 0) {
					notice = "You don't have any default language model set. Go to Config to set one.";
				} else {
					goto('/qa');
				}
			}
		})
		.catch((error) => console.error(error));

	// let defaultModel: string | null = null;
	// (async () => {
	// 	async function checkDefaultModel(): Promise<string | null> {
	// 		const store = new Store('.settings.dat');
	// 		return await store.get<string>('default_model');
	// 	}

	// 	defaultModel = await checkDefaultModel();
	// })();

	// let lala = '';
	// listen<string>('log', (event) => {
	// 	lala = event.payload;
	// });
</script>

<div class="text-center h-screen">
	{#if notice != ''}
		<h2 class=" font-bold tracking-tight">{notice}</h2>
		<button type="button" class="btn variant-filled" on:click={() => goto('/config')}
			>Go to configuration</button
		>
	{/if}
</div>

<!-- <a href="/config">Elegir modelo</a>
<p>{lala}</p>
<input class="input" bind:value={lala} />
<button
	on:click={async () => {
		await invoke('log', { message: 'hello' });
	}}>click</button
> -->
