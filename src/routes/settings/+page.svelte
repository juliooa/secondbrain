<script lang="ts">
	import type { CommandResponseLanguagesModels, LanguageModel } from '$lib/types';
	import { invoke } from '@tauri-apps/api';
	import { listen } from '@tauri-apps/api/event';
	import 'iconify-icon';
	import { modalStore } from '@skeletonlabs/skeleton';
	import type { ModalSettings } from '@skeletonlabs/skeleton';
	import { getVersion } from '@tauri-apps/api/app';

	let current_model: LanguageModel | null = null;

	let showNotice: boolean = false;
	let noticeMessage = '';
	let noticeBg: string = 'variant-filled-success';

	let selectedModelFilename: string = '';

	let languageModels: LanguageModel[] = [];
	let selectedModelToDownloadFilename: string = '';

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

	getPrompt();
</script>

<div class="p-4 h-full flex flex-col">
	<h1 class="h1">Configuration</h1>
	<div class="flex flex-col justify-between grow mt-4">
		<div>
			<p class="mt-3">Active Language Model</p>
			<h4 class="variant-ringed-tertiary p-2">
				{#if current_model}
					<h4>{current_model?.filename}</h4>
				{:else}
					<span class="text-red-400"> No model set. Choose or download one.</span>
				{/if}
			</h4>
			<label class="label">
				<span>Prompt</span>
				<textarea class="textarea" rows="4" placeholder="Enter some long form content."
					>{prompt}
				</textarea>
			</label>
		</div>
	</div>
</div>
