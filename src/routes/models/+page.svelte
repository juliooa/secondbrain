<script lang="ts">
	import type { CommandResponseLanguagesModels, DownloadProgress, LanguageModel } from '$lib/types';
	import { tauri } from '@tauri-apps/api';
	import * as toasts from '$lib/toasts';
	import 'iconify-icon';
	import { getVersion } from '@tauri-apps/api/app';
	import LanguageModelRow from '$lib/components/LanguageModelRow.svelte';

	let modelsFolder: string = '';
	let current_model: LanguageModel | null = null;

	let languageModels: LanguageModel[] = [];

	function getLanguageModels() {
		tauri
			.invoke<CommandResponseLanguagesModels>('get_language_models')
			.then((result) => {
				console.log('Language models:' + JSON.stringify(result));
				languageModels = result.models;
				current_model = languageModels.filter((model) => model.current == true)[0];
				console.log('Language models javascript:' + JSON.stringify(languageModels));
				console.log('Current model:' + JSON.stringify(current_model));
			})
			.catch((error) => {
				console.log('Error:' + error);
				toasts.error('Error getting language models: ' + error);
			});
	}

	async function showInFolder(path: string) {
		await tauri.invoke('show_in_folder', { path });
	}

	async function selectFolder() {
		try {
			const result = await tauri.invoke<string>('choose_directory');
			modelsFolder = result;
			getLanguageModels();
		} catch (e) {
			console.log(e);
		}
	}

	async function refreshFolder() {
		getLanguageModels();
	}

	async function getModelsFolder() {
		try {
			modelsFolder = await tauri.invoke<string>('get_models_folder');
		} catch (e) {
			console.log(e);
		}
	}

	getLanguageModels();
	getModelsFolder();

	let app_version = '';
	getVersion().then((version) => {
		app_version = 'v' + version;
	});

	function getDownloadInProgress() {
		return tauri
			.invoke<DownloadProgress>('get_download_progress')
			.then((result) => {
				console.log('Download progress:' + JSON.stringify(result));
				return result;
			})
			.catch((error) => {
				console.log('Error:' + error);
				return null;
			});
	}
</script>

<div class="p-4 flex flex-col h-screen">
	<h1 class="h1">Models Management</h1>
	<div class="flex flex-col justify-between mt-4">
		<p class="mt-3">Models folder:</p>
		<div class="flex">
			<input
				type="text"
				bind:value={modelsFolder}
				disabled
				class="input
			disabled:text-white"
				style="opacity: 1 !important;"
			/>
			<button
				type="button"
				class="btn variant-filled-secondary ml-1"
				on:click={() => selectFolder()}>Change</button
			>
			<button
				type="button"
				class="btn variant-filled-secondary ml-1"
				on:click={() => refreshFolder()}>Refresh</button
			>
		</div>
	</div>
	<h3 class="h3 mt-3">Current Language Model:</h3>
	<div>
		<h4 class="variant-ringed-tertiary p-2">
			{#if current_model}
				{#if current_model.has_info}
					({current_model.info.name})
				{:else}
					<h4>{current_model.filename}</h4>
				{/if}
			{:else}
				<span class="text-red-400"> No model set. Choose or download one.</span>
			{/if}
		</h4>
	</div>
	<h3 class="h3 mt-4 mb-1">Models</h3>
	<div class="overflow-auto">
		{#each languageModels as model}
			<LanguageModelRow {model} />
		{/each}
	</div>
	<div />
</div>
