<script lang="ts">
	import type { CommandResponseLanguagesModels, LanguageModel } from '$lib/types';
	import { ProgressBar, filter } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api';
	import { listen } from '@tauri-apps/api/event';
	import 'iconify-icon';
	import { modalStore } from '@skeletonlabs/skeleton';
	import type { ModalSettings, ModalComponent } from '@skeletonlabs/skeleton';

	interface DownloadProgress {
		modelId: number;
		progress: number;
		total: number;
	}

	let current_model: LanguageModel | null = null;
	let downloadProgress: DownloadProgress | null = null;

	let showNotice: boolean = false;
	let noticeMessage = '';
	let noticeBg: string = 'variant-filled-success';

	let selectedModelId: number = 0;
	let isDownloading: boolean = false;

	let languageModels: LanguageModel[] = [];
	let selectedModelToDownloadId: number = 0;

	listen<DownloadProgress>('progress_download', (progress) => {
		console.log('Progress download:' + JSON.stringify(progress));
		if (downloadProgress == null) {
			downloadProgress = {
				modelId: progress.payload.modelId,
				progress: 0,
				total: 0
			};
		}
		downloadProgress.progress += formatBytesToMegabytes(progress.payload.progress);
		downloadProgress.total = formatBytesToMegabytes(progress.payload.total);
		if (downloadProgress.progress >= downloadProgress.total) {
			isDownloading = false;
		}
	});

	listen<string>('finish_download', (event) => {
		isDownloading = false;
		downloadProgress = null;
		showNoticeAlert(event.payload);

		getLanguageModels();
	});

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
			.catch((error) => {
				showErrorAlert('Error:' + error);
			});
	}

	function getLanguageModels() {
		invoke<CommandResponseLanguagesModels>('get_language_models')
			.then((result) => {
				console.log('Language models:' + JSON.stringify(result));
				languageModels = result.models;
				current_model = languageModels.filter((model) => model.current == true)[0];
			})
			.catch((error) => {
				showErrorAlert('Error:' + error);
			});
	}

	function closeNotice() {
		showNotice = false;
	}

	function activate_model() {
		let selectedModel = languageModels.filter((model) => model.id == selectedModelId)[0];
		invoke('set_current_model', {
			modelId: selectedModel.id,
			modelFilename: selectedModel.filename,
			modelName: selectedModel.name,
			finishDownloadNotice: `Finish downloading model ${selectedModel.name}`
		})
			.then(() => {
				console.log('Model activated:' + selectedModel.name);
				showNoticeAlert(selectedModel.name + ' is the active model.');
				getLanguageModels();
			})
			.catch((error) => {
				console.log('Error:' + error);
				showErrorAlert('Error:' + error);
			});
	}
	function delete_model() {
		const modal: ModalSettings = {
			type: 'confirm',
			title: 'Please Confirm',
			modalClasses: '!bg-red-500',
			buttonTextConfirm: 'Yes, delete it',
			body: 'This will delete the file from your disk. To use it again you will need to download it. Are you sure?',
			response: (confirm: boolean) => {
				if (confirm) {
					call_delete_model_command(selectedModelId);
				}
			}
		};
		modalStore.trigger(modal);
	}
	function showNoticeAlert(message: string) {
		noticeMessage = message;
		showNotice = true;
		noticeBg = 'variant-filled-success';
	}

	function showErrorAlert(error: string) {
		noticeBg = 'variant-filled-error';
		noticeMessage = error;
		showNotice = true;
	}

	function formatBytesToMegabytes(bytes: number): number {
		return bytes / (1024 * 1024);
	}

	getLanguageModels();

	function cancelDownload() {
		invoke<number>('cancel_download', {
			modelId: downloadProgress?.modelId
		})
			.then((canceledModelId) => {
				isDownloading = false;
				downloadProgress = null;
				call_delete_model_command(canceledModelId);
			})
			.catch((error) => {
				console.log('Error:' + error);
				showErrorAlert('Error:' + error);
			});
	}

	function call_delete_model_command(modelId: number) {
		let selectedModel = languageModels.filter((model) => model.id == modelId)[0];
		invoke('delete_model', {
			modelId: selectedModel.id,
			modelFilename: selectedModel.filename,
			finishDownloadNotice: `Model ${selectedModel.name} deleted`
		})
			.then(() => {
				showNoticeAlert(`Model ${selectedModel.name} deleted`);
				getLanguageModels();
			})
			.catch((error) => {
				console.log('Error:' + error);
				showErrorAlert('Error:' + error);
			});
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
				<div class="flex flex-row justify-between">
					<button
						type="button"
						class="btn variant-filled-secondary"
						on:click={() => activate_model()}>Activate model</button
					>
					<button type="button" class="btn variant-filled-error" on:click={() => delete_model()}
						>Delete model</button
					>
				</div>
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
		<div class="card">
			{#if isDownloading}
				<div class="m-3">
					<div class="flex flex-row justify-between">
						<div>
							<h4>Downloading...</h4>
							<p>
								Progress: {(downloadProgress?.progress || 0).toLocaleString(undefined, {
									maximumFractionDigits: 2
								})} Mb
							</p>
							<p>
								Total: {(downloadProgress?.total || 0).toLocaleString(undefined, {
									maximumFractionDigits: 2
								})} Mb
							</p>
						</div>
						<div>
							<button class="btn-icon variant-filled btn-icon-sm" on:click={cancelDownload}>
								<iconify-icon width="22" icon="ion:close" />
							</button>
						</div>
					</div>
					<div>
						<ProgressBar
							max={downloadProgress?.total}
							min={0}
							value={downloadProgress?.progress}
							height="h-3"
							meter="variant-filled-success"
						/>
					</div>
				</div>
			{/if}
			{#if showNotice}
				<aside class="alert {noticeBg} flex flex-row">
					<div class="alert-message grow">
						<h4>{noticeMessage}</h4>
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
