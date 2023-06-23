<script lang="ts">
	import type { PageData } from './$types';
	import * as llm from '$lib/llm';
	import * as toasts from '$lib/toasts';
	import type { NewTokenPayload, TextBlock } from '$lib/types';
	import { CodeBlock, ProgressBar, toastStore } from '@skeletonlabs/skeleton';
	import { listen } from '@tauri-apps/api/event';
	import { parseText } from '$lib/utils';

	export let data: PageData;

	let query = '';
	let current_query = '';
	let incomingMessage: string = '';
	let parsedTextBlocks: TextBlock[] = [];
	let loading = false;

	listen<NewTokenPayload>('new_token', (event) => {
		console.log(incomingMessage);
		incomingMessage = incomingMessage + event.payload.message;
		parsedTextBlocks = parseText(incomingMessage);
	});

	async function askModel() {
		parsedTextBlocks = [];
		loading = true;
		let answer: string = await llm.ask(query);
		console.log(answer);
		loading = false;
		query = '';
		incomingMessage = '';
	}

	function cancelInference() {
		loading = false;
		llm
			.cancelInference()
			.then(() => {
				console.log('Inference cancelled');
			})
			.catch((error) => {
				console.log('Error cancelling inference: ' + error);
				toasts.error('Error cancelling inference: ' + error);
			});
	}
</script>

<div class="h-full">
	<div class="p-4">
		<h2>Ask the model</h2>
		<div class="flex items-center">
			{#if data.activeModel != null}
				<p class="text-xl text-warning-400">{data.activeModel.name}</p>
			{:else}
				<p class="text-xl text-error-500">No active model</p>
			{/if}
		</div>
	</div>
	<div class="p-4">
		<div class="flex justify-center items-center">
			<div class="card h-full w-full">
				<div class="p-4 md:p-10">
					<form
						class="flex"
						on:submit|preventDefault={() => {
							askModel();
						}}
					>
						<input
							class="input text-xl"
							type="text"
							placeholder="your question here"
							bind:value={query}
							disabled={loading}
						/>

						{#if loading}
							<button
								type="submit"
								class="btn variant-filled-error w-1/5 ml-4 text-xl"
								on:click={() => {
									cancelInference();
								}}>Cancel</button
							>
						{:else}
							<button
								type="submit"
								class="btn variant-filled-secondary w-1/5 ml-4 text-xl"
								on:click={() => {
									current_query = query;
									askModel();
								}}>Ask</button
							>
						{/if}
					</form>

					{#if loading}
						<div class="mt-5">
							<ProgressBar height="h-3" meter="bg-warning-500" />
						</div>
					{/if}
					{#if current_query.length > 0}
						<div>
							<h4 class="text-xxl mb-2 mt-6">Question</h4>
							<div class="flex flex-col rounded-[8px] bg-tertiary-500 p-4 text-xl">
								<p style="white-space: pre-line;">{current_query}</p>
							</div>
						</div>
					{/if}
					{#if parsedTextBlocks.length > 0}
						<div>
							<h4 class="text-xxl mb-2 mt-6">Answer</h4>
							<div class="flex flex-col rounded-[8px] bg-tertiary-500 p-4 text-xl">
								{#each parsedTextBlocks as textBlock}
									{#if textBlock.isCodeBlock}
										<CodeBlock language={textBlock.language} code={textBlock.text} />
									{:else}
										<p style="white-space: pre-line;">{textBlock.text}</p>
									{/if}
								{/each}
							</div>
						</div>
					{/if}
				</div>
			</div>
		</div>
	</div>
</div>
