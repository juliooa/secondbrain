<script lang="ts">
	import { infere } from '$lib/llm';
	import type { NewTokenPayload } from '$lib/types';
	import { CodeBlock, ProgressBar } from '@skeletonlabs/skeleton';
	import { listen } from '@tauri-apps/api/event';
	import { text } from 'svelte/internal';

	let query = '';
	let current_query = '';
	let incomingMessage: string = '';
	let parsedTextBlocks: TextBlock[] = [];
	let loading = false;
	interface TextBlock {
		isCodeBlock: boolean;
		text: string;
		language?: string;
	}

	function parseText(text: string): TextBlock[] {
		const regex = /```([\w-]+)?\s*([\s\S]+?)\s*```/g;
		const blocks: TextBlock[] = [];
		let lastIndex = 0;
		let match;

		while ((match = regex.exec(text))) {
			const [fullMatch, language, code] = match;
			const preMatch = text.slice(lastIndex, match.index);
			if (preMatch) {
				blocks.push({ isCodeBlock: false, text: preMatch });
			}
			blocks.push({ isCodeBlock: true, text: code, language });
			lastIndex = match.index + fullMatch.length;
		}

		const lastBlock = text.slice(lastIndex);
		if (lastBlock) {
			blocks.push({ isCodeBlock: false, text: lastBlock });
		}

		return blocks;
	}

	listen<NewTokenPayload>('new_token', (event) => {
		console.log(incomingMessage);
		incomingMessage = incomingMessage + event.payload.message;
		parsedTextBlocks = parseText(incomingMessage);
	});

	async function ask_model() {
		parsedTextBlocks = [];
		loading = true;
		let answer: string = await infere(query);
		console.log('answer');
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
</script>

<div class="p-4 h-screen">
	<h1 class="h1">Ask the model</h1>
	<div class="flex justify-center items-center mt-8">
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
