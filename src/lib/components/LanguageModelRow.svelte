<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { TAURI_COMMAND } from '$lib/commands';
	import type { DownloadProgress, LanguageModel } from '$lib/types';
	import { modalStore, type ModalSettings, ProgressRadial } from '@skeletonlabs/skeleton';
	import { tauri } from '@tauri-apps/api';
	import { listen } from '@tauri-apps/api/event';
	import * as toasts from '$lib/toasts';

	export let model: LanguageModel;
	let arquitecture: string = 'llama';
	let downloadProgress: DownloadProgress | null = null;

	async function downloadModel() {
		model.isDownloading = true;
		tauri
			.invoke(TAURI_COMMAND.DOWNLOAD_MODEL, {
				url: model.info.url,
				modelFilename: model.filename,
				finishDownloadNotice: `Finish downloading model ${model.info.name}`
			})
			.then((result) => console.log(result))
			.catch((error) => {
				console.log('Error: ' + error);
			});
	}

	function activateModel() {
		tauri
			.invoke('set_current_model', {
				modelFilename: model.filename,
				modelName: model.has_info ? model.info.name : model.filename,
				modelArquitecture: arquitecture
			})
			.then(() => {
				model.current = true;
				invalidate('app:activeModelChanged');
				toasts.success('Model activated');
			})
			.catch((error) => {
				toasts.error('Error activating model: ' + error);
				console.log('Error:' + error);
			});
	}

	function callDeleteModelCommand() {
		tauri
			.invoke('delete_model', {
				modelFilename: model.filename,
				finishDownloadNotice: `Model ${model.info.name} deleted`
			})
			.then(() => {
				model.downloaded = false;
				toasts.success('Model deleted');
			})
			.catch((error) => {
				console.log('Error:' + error);
				toasts.error('Error deleting model: ' + error);
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
					callDeleteModelCommand();
				}
			}
		};
		modalStore.trigger(modal);
	}

	listen<DownloadProgress>('progress_download', (progress) => {
		if (progress.payload.model_filename != model.filename) {
			return;
		}
		model.downloaded = false;
		model.isDownloading = true;
		model = model;
		console.log('Progress download:' + JSON.stringify(progress));
		if (downloadProgress == null) {
			downloadProgress = {
				model_filename: progress.payload.model_filename,
				progress: 0,
				total: 0,
				percentage: 0
			};
		}
		downloadProgress.progress = formatBytesToMegabytes(progress.payload.progress);
		downloadProgress.total = formatBytesToMegabytes(progress.payload.total);
		downloadProgress.percentage = Math.round(
			(downloadProgress.progress / downloadProgress.total) * 100
		);
		if (downloadProgress.progress >= downloadProgress.total) {
			model.isDownloading = false;
			model.downloaded = true;
		}
	});

	listen<string>('finish_download', (event) => {
		model.isDownloading = false;
		model.downloaded = true;
		downloadProgress = null;
		toasts.success('Model downloaded!');
	});

	function formatBytesToMegabytes(bytes: number): number {
		return bytes / (1024 * 1024);
	}

	function cancelDownload() {
		console.log('Cancel download:' + downloadProgress?.model_filename);
		tauri
			.invoke<String>('cancel_download', {
				modelFilename: downloadProgress?.model_filename
			})
			.then((canceledModelFilename) => {
				model.isDownloading = false;
				downloadProgress = null;
				callDeleteModelCommand();
			})
			.catch((error) => {
				toasts.error('Error canceling download: ' + error);
			});
	}
</script>

<div class="flex flex-col bg-tertiary-800 rounded-lg mb-3 mr-2">
	<div class="flex flex-row items-center px-4 py-4">
		<div class="flex flex-row items-center grow">
			<div class="flex flex-col">
				{#if model.has_info}
					<div class="text-xl font-medium">
						{model.info.name}
					</div>
					<div class="text-sm text-gray-200">
						{model.filename}
					</div>
					<div class="text-sm text-gray-200">
						Size: {model.info.size}
					</div>
				{:else}
					<div class="text-xl font-medium">
						{model.filename}
					</div>
				{/if}
			</div>
		</div>

		{#if model.downloaded}
			<select class="select h-12 w-32" bind:value={arquitecture} disabled={model.current}>
				<option value="llama">llama</option>
				<option value="bloom">bloom</option>
				<option value="gpt2">gpt2</option>
				<option value="gptj">gptj</option>
				<option value="gptneox">gptneox</option>
				<option value="mpt">mpt</option>
			</select>
			{#if model.current}
				<button type="button" class="btn h-9 w-28 variant-filled-warning text-white ml-1" disabled
					>Active</button
				>
			{:else}
				<button
					type="button"
					class="btn h-9 w-28 variant-filled-success text-white ml-1"
					on:click={() => {
						activateModel();
					}}>Activate</button
				>
			{/if}
			<button type="button" class="btn h-9 variant-filled-error ml-1" on:click={() => deleteModel()}
				><iconify-icon width="22" icon="ion:close" /></button
			>
		{:else if model.isDownloading}
			<div class="flex flex-col items-end">
				<ProgressRadial
					stroke={100}
					value={downloadProgress?.percentage}
					meter="stroke-primary-500"
					track="stroke-primary-500/30"
					font={100}
					width="w-16">{downloadProgress?.percentage}%</ProgressRadial
				>
				<div class="flex flex-row mt-1">
					<div class=" w-28 text-right">
						{Math.trunc(downloadProgress?.progress || 0).toLocaleString(undefined, {
							maximumFractionDigits: 2
						})} Mb
					</div>
					<div>
						/ {(downloadProgress?.total || 0).toLocaleString(undefined, {
							maximumFractionDigits: 2
						})} Mb
					</div>
				</div>
			</div>
			<button class="btn-icon variant-filled btn-icon-sm ml-2" on:click={cancelDownload}>
				<iconify-icon width="22" icon="ion:close" />
			</button>
		{:else}
			<button
				type="button"
				class="btn variant-filled-secondary ml-1"
				on:click={() => downloadModel()}
				><iconify-icon width="22" icon="ion:cloud-download" /></button
			>
		{/if}
	</div>
</div>
