<script lang="ts">
	import type { CommandResponseLanguagesModels, LanguageModel } from '$lib/types';
	import { ProgressBar } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api';
	import { listen } from '@tauri-apps/api/event';
	import 'iconify-icon';
	import { modalStore } from '@skeletonlabs/skeleton';
	import type { ModalSettings } from '@skeletonlabs/skeleton';
	import { getVersion } from '@tauri-apps/api/app';

	interface DownloadProgress {
		model_filename: string;
		progress: number;
		total: number;
	}

	let current_model: LanguageModel | null = null;
	let downloadProgress: DownloadProgress | null = null;

	let showNotice: boolean = false;
	let noticeMessage = '';
	let noticeBg: string = 'variant-filled-success';

	let selectedModelFilename: string = '';
	let isDownloading: boolean = false;

	let languageModels: LanguageModel[] = [];
	let selectedModelToDownloadFilename: string = '';

	listen<DownloadProgress>('progress_download', (progress) => {
		console.log('Progress download:' + JSON.stringify(progress));
		if (downloadProgress == null) {
			downloadProgress = {
				model_filename: progress.payload.model_filename,
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
		let selectedModel = languageModels.filter(
			(model) => model.filename == selectedModelToDownloadFilename
		)[0];
		console.log('Prepare to download:' + selectedModel);
		isDownloading = true;

		invoke('download_model', {
			url: selectedModel.url,
			modelFilename: selectedModel.filename,
			finishDownloadNotice: `Finish downloading model ${selectedModel.name}`
		})
			.then((result) => console.log(result))
			.catch((error) => {
				showErrorAlert('Error:' + error);
			});
	}

	function activateModel() {
		let selectedModel = languageModels.filter(
			(model) => model.filename == selectedModelFilename
		)[0];
		console.log(selectedModel);

		// If the promptBase is empty, the invoke method fails
		// because it thinks the promptBase property is not present.
		let promptBase = '[[message]]';
		if (selectedModel.prompt_base.length > 0) {
			promptBase = selectedModel.prompt_base;
		}
		invoke('set_current_model', {
			modelFilename: selectedModel.filename,
			modelName: selectedModel.name,
			modelArquitecture: selectedModel.arquitecture,
			promptBase: promptBase
		})
			.then(() => {
				showNoticeAlert(selectedModel.name + ' is the active model.');
				getLanguageModels();
				getPrompt();
			})
			.catch((error) => {
				console.log('Error:' + error);
				showErrorAlert('Error:' + error);
			});
	}

	function deleteModel() {
		const modal: ModalSettings = {
			type: 'confirm',
			title: 'Please Confirm',
			modalClasses: '!bg-red-500',
			buttonTextConfirm: 'Yes, delete it',
			body: 'This will delete the file from your disk. To use it again you will need to download it. Are you sure?',
			response: (confirm: boolean) => {
				if (confirm) {
					callDeleteModelCommand(selectedModelFilename);
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

	function cancelDownload() {
		console.log('Cancel download:' + downloadProgress?.model_filename);
		invoke<String>('cancel_download', {
			modelFilename: downloadProgress?.model_filename
		})
			.then((canceledModelFilename) => {
				isDownloading = false;
				downloadProgress = null;
				callDeleteModelCommand(canceledModelFilename);
			})
			.catch((error) => {
				console.log('Error:' + error);
				showErrorAlert('Error:' + error);
			});
	}

	function callDeleteModelCommand(modelFilename: String) {
		let selectedModel = languageModels.filter((model) => model.filename == modelFilename)[0];
		invoke('delete_model', {
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

	let prompt = '';
	let getPrompt = () => {
		invoke<string>('get_prompt_base')
			.then((result) => {
				prompt = result;
			})
			.catch((error) => {
				showErrorAlert('Error:' + error);
			});
	};

	function getLanguageModels() {
		invoke<CommandResponseLanguagesModels>('get_language_models')
			.then((result) => {
				console.log('Language models:' + JSON.stringify(result));
				languageModels = result.models;
				current_model = languageModels.filter((model) => model.current == true)[0];
				console.log('Language models javascript:' + JSON.stringify(languageModels));
				console.log('Current model:' + JSON.stringify(current_model));
			})
			.catch((error) => {
				showErrorAlert('Error:' + error);
			});
	}

	getLanguageModels();
	getPrompt();

	let app_version = '';
	getVersion().then((version) => {
		app_version = 'v' + version;
	});
</script>

<div class="p-4 h-screen flex flex-col">
	<h1 class="h1">Configuration</h1>
	<div class="flex flex-col justify-between grow mt-4">
		<div>
			<label class="label">
				<span>Prompt</span>
				<textarea class="textarea" rows="4" placeholder="Enter some long form content."
					>{prompt}
				</textarea>
			</label>

			<p class="mt-3">Current Language Model:</p>
			<h4 class="variant-ringed-tertiary p-2">
				{#if current_model}
					<h4>{current_model?.name}</h4>
				{:else}
					<span class="text-red-400"> No model set. Choose or download one.</span>
				{/if}
			</h4>

			<label class="label mt-4">
				<span>Available language models</span>
				<select class="select" size="6" bind:value={selectedModelFilename}>
					{#each languageModels.filter((model) => model.downloaded) as model}
						<option value={model.filename} disabled={model.current}
							>{model.name} {model.current ? '(current)' : ''}</option
						>
					{/each}
				</select>
				<div class="flex flex-row justify-between">
					<button
						type="button"
						class="btn variant-filled-secondary"
						on:click={() => activateModel()}>Activate model</button
					>
					<button type="button" class="btn variant-filled-error" on:click={() => deleteModel()}
						>Delete model</button
					>
				</div>
			</label>
			<label class="label mt-4">
				<span
					>Available models to download {languageModels.length > 0
						? '(' + languageModels.length + ')'
						: ''}</span
				>
				<select class="select" size="6" bind:value={selectedModelToDownloadFilename}>
					{#each languageModels.filter((model) => !model.downloaded) as model}
						<option value={model.filename} disabled={model.current}>{model.name}</option>
					{/each}
				</select>
				<button type="button" class="btn variant-filled-secondary" on:click={() => downloadModel()}
					>Download selected</button
				>
			</label>
		</div>
		<div class="card mt-4">
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
						<p class="mt-2 text-sm">
							(Please do not leave this window until the download is complete, we are still working
							to make this more mmm... smart.)
						</p>
					</div>
				</div>
			{/if}
			{#if showNotice}
				<aside class="alert {noticeBg} flex flex-row">
					<div class="alert-message grow">
						<h4>{noticeMessage}</h4>
					</div>
					<div class="alert-actions">
						<button
							class="btn-icon variant-filled"
							on:click={() => {
								showNotice = false;
							}}
						>
							<iconify-icon width="28" icon="ion:close" />
						</button>
					</div>
				</aside>
			{/if}
		</div>
		<div class="text-right w-full text-gray-800">{app_version}</div>
	</div>
</div>
