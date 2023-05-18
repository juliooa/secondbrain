<script lang="ts">
	//import { MessageRole, type Message } from '../types';
	export let message: Message;

	import { CodeBlock } from '@skeletonlabs/skeleton';
	import type { Message } from '../types';

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

	let parsedTextBlocks = parseText(message.text);
</script>

<div
	class="pt-2 pb-2 {message.role == 'human'
		? 'variant-glass-tertiary-500'
		: 'variant-soft-tertiary'}"
>
	<div class="text-xl flex flex-row m-3">
		<span class="chip variant-filled flex-none h-5 w-20 mt-1">{message.role.toUpperCase()}</span>
		<div class="grow ml-3">
			{#each parsedTextBlocks as textBlock}
				{#if textBlock.isCodeBlock}
					<CodeBlock language={textBlock.language} code={textBlock.text} />
				{:else}
					<p style="white-space: pre-line;">{textBlock.text.trimStart()}</p>
				{/if}
			{/each}
		</div>
	</div>
</div>
