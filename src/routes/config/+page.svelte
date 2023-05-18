<script lang="ts">
	import type { CommandResponseLanguagesModels, LanguageModel } from '$lib/types';
	import { ProgressBar, filter } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api';
	import { listen } from '@tauri-apps/api/event';
	import 'iconify-icon';

	interface ProgressPayload {
		id: number;
		progress: number;
		total: number;
	}

	let current_model: LanguageModel | null = null;
	let downloadComplete: boolean = false;
	let downloadFinishNotice = '';
	let selectedModelId: number = 0;
	let isDownloading: boolean = false;
	let progress: number = 0;
	let total: number = 0;
	let languageModels: LanguageModel[] = [];
	let selectedModelToDownloadId: number = 0;
	async function downloadModel() {
		let selectedModel = languageModels.filter((model) => model.id == selectedModelToDownloadId)[0];
		console.log('Prepare to download:' + selectedModel);
		isDownloading = true;
		invoke('download_model', {
			url: selectedModel.url,
			modelId: selectedModel.id,
			fileName: selectedModel.filename,
			finishDownloadNotice: `Finish downloading model ${selectedModel.name}`
		})
			.then((result) => console.log(result))
			.catch((error) => console.error(error));
	}

	listen<ProgressPayload>('progress_download', (progressPayload) => {
		progress += progressPayload.payload.progress;
		total = progressPayload.payload.total;
		if (progressPayload.payload.progress >= progressPayload.payload.total) {
			isDownloading = false;
		}
	});

	listen<string>('finish_download', (event) => {
		console.log('Finish download:' + event.payload);
		downloadFinishNotice = event.payload;
		downloadComplete = true;
		isDownloading = false;
		getLanguageModels();
	});

	function getLanguageModels() {
		invoke<CommandResponseLanguagesModels>('get_language_models')
			.then((result) => {
				console.log('Language models:' + JSON.stringify(result));
				languageModels = result.models;
				current_model = languageModels.filter((model) => model.current == true)[0];
			})
			.catch((error) => console.error(error));
	}

	getLanguageModels();

	function closeNotice() {
		downloadComplete = false;
	}

	function activate_model() {
		let selectedModel = languageModels.filter((model) => model.id == selectedModelId)[0];
		invoke('set_current_model', {
			modelId: selectedModel.id,
			modelFilename: selectedModel.filename,
			finishDownloadNotice: `Finish downloading model ${selectedModel.name}`
		})
			.then((result) => {
				console.log(result);
				getLanguageModels();
			})
			.catch((error) => console.error(error));
	}
</script>

<div class="p-4 h-screen flex flex-col">
	<h1 class="h1">Configuration</h1>
	<div class="flex flex-col justify-between grow mt-4">
		<div>
			<span>Current Language Model</span>
			{#if current_model}
				<h4>{current_model?.name}</h4>
			{:else}
				<h4>No language set. Choose one or download one.</h4>
			{/if}
			<label class="label mt-4">
				<span>Available language models</span>
				<select class="select" size="6" bind:value={selectedModelId}>
					{#each languageModels.filter((model) => model.downloaded) as model}
						<option value={model.id} disabled={model.current}>{model.name}</option>
					{/each}
				</select>
				<button type="button" class="btn variant-filled-secondary" on:click={() => activate_model()}
					>Activate model</button
				>
			</label>
			<label class="label mt-4">
				<span>Available models to download</span>
				<select class="select" size="6" bind:value={selectedModelToDownloadId}>
					{#each languageModels.filter((model) => !model.downloaded) as model}
						<option value={model.id} disabled={model.current}>{model.name}</option>
					{/each}
				</select>
				<button type="button" class="btn variant-filled-secondary" on:click={() => downloadModel()}
					>Download selected</button
				>
			</label>
		</div>
		<div>
			{#if isDownloading}
				<p>Downloading...</p>
				<p>Progress: {progress}</p>
				<p>Total: {total}</p>
				<ProgressBar max={total} min={0} value={progress} />
			{/if}
			{#if downloadComplete}
				<aside class="alert variant-filled-success">
					<div class="alert-message">
						<h3 class="h3">{downloadFinishNotice}</h3>
					</div>
					<div class="alert-actions">
						<button class="btn-icon variant-filled" on:click={closeNotice}>
							<iconify-icon width="28" icon="ion:close" />
						</button>
					</div>
				</aside>
			{/if}
		</div>
	</div>
</div>
