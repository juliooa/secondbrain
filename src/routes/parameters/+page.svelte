<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import type { PageData } from './$types';
	import 'iconify-icon';
	import * as toasts from '$lib/toasts';
	export let data: PageData;

	type InferenceParameters = {
		top_p: string;
		top_k: string;
		temperature: string;
		repetition_penalty: string;
		prompt_template: string;
	};

	let promptTemplate = '';
	let topP = '';
	let topK = '';
	let temperature = '';
	let repetitionPenalty = '';

	function saveParameters() {
		console.log(temperature);
		invoke('save_parameters', {
			promptTemplate,
			temperature: temperature.toString(),
			topP: topP.toString(),
			topK: topK.toString(),
			repetitionPenalty: repetitionPenalty.toString()
		})
			.then((result) => {
				console.log(result);
				toasts.success('Parameters saved');
			})
			.catch((error) => {
				toasts.error('Failed to save parameters:' + error);
			});
	}

	invoke<InferenceParameters>('get_parameters')
		.then((result) => {
			topP = result.top_p;
			topK = result.top_k;
			temperature = result.temperature;
			repetitionPenalty = result.repetition_penalty;
			promptTemplate = result.prompt_template;
		})
		.catch((error) => {
			toasts.error('Failed to get parameters:' + error);
		});
</script>

<div class="p-4 h-full flex flex-col">
	<h1 class="h1">Inference Parameters</h1>
	<div class="flex flex-col justify-between grow mt-4">
		<div>
			<p class="mt-3">Active Language Model</p>
			<h4 class="variant-ringed-tertiary p-2 mb-2">
				{#if data.activeModel != null}
					<p class="text-xl text-warning-400">{data.activeModel.name}</p>
				{:else}
					<p class="text-xl text-error-500">No active model</p>
				{/if}
			</h4>
			<label class="label">
				<span>Prompt</span>
				<textarea class="textarea" rows="4" placeholder="Your prompt" bind:value={promptTemplate} />
			</label>
			<div class="flex flex-col">
				<div class="flex flex-row items-center">
					<span class="w-28">Temperature</span>
					<input class="input w-36" type="number" bind:value={temperature} />
				</div>
				<div class="flex flex-row items-center mt-1">
					<span class="w-28">Top P</span>
					<input class="input w-36" type="number" bind:value={topP} />
				</div>
				<div class="flex flex-row items-center mt-1">
					<span class="w-28">Top K</span>
					<input class="input w-36" type="number" bind:value={topK} />
				</div>
				<div class="flex flex-row items-center mt-1">
					<span class="w-28">Repeat Penalty</span>
					<input class="input w-36" type="number" bind:value={repetitionPenalty} />
				</div>
			</div>
			<div class="flex flex-row items-center mt-1">
				<button class="btn variant-filled-secondary mt-2" type="submit" on:click={saveParameters}>
					Save
				</button>
			</div>
		</div>
	</div>
</div>
