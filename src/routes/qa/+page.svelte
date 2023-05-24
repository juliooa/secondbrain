<script lang="ts">
	import { ask } from '$lib/llm';
	import type { NewTokenPayload, TextBlock } from '$lib/types';
	import { CodeBlock, ProgressBar } from '@skeletonlabs/skeleton';
	import { listen } from '@tauri-apps/api/event';
	import { getCurrentModel } from '$lib/local_store';
	import { parseText } from '$lib/utils';

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

	async function ask_model() {
		parsedTextBlocks = [];
		loading = true;
		let answer: string = await ask(query);
		console.log(answer);
		loading = false;
		query = '';
		incomingMessage = '';
	}

	function cancel_inference() {
		loading = false;
		parsedTextBlocks = [];
		incomingMessage = '';
		current_query = '';
	}

	async function getModelName() {
		getCurrentModel().then((currenModel) => {
			if (currenModel) {
				console.log(currenModel.name);
				modelName = currenModel.name;
			}
		});
	}
	getModelName();
	let modelName: string;
</script>

<div class="h-screen">
	<div class="p-4">
		<h2>Ask the model</h2>
		<div class="flex items-center">
			{#if modelName != null}
				<p class="text-xl text-warning-400">{modelName}</p>
			{:else}
				<p class="text-xl text-error-500">No active model</p>
			{/if}
		</div>
	</div>
	<div class="p-4">
		<div class="flex justify-center items-center">
			<div class=" card h-full w-full">
				<div class="p-4 md:p-10">
					<form
						class="flex"
						on:submit|preventDefault={() => {
							ask_model();
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
									cancel_inference();
								}}>Cancel</button
							>
						{:else}
							<button
								type="submit"
								class="btn variant-filled-secondary w-1/5 ml-4 text-xl"
								on:click={() => {
									current_query = query;
									ask_model();
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
