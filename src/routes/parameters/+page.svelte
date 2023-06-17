<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import type { PageData } from './$types';
	import 'iconify-icon';
	export let data: PageData;

	let prompt = '';
	let getPrompt = () => {
		invoke<string>('get_prompt_base')
			.then((result) => {
				prompt = result;
			})
			.catch((error) => {});
	};

	getPrompt();
</script>

<div class="p-4 h-full flex flex-col">
	<h1 class="h1">Model Settings</h1>
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
				<textarea class="textarea" rows="4" placeholder="Your prompt">{prompt} </textarea>
			</label>
		</div>
	</div>
</div>
