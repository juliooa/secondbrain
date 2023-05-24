<script lang="ts">
	export let message: Message;

	import { CodeBlock } from '@skeletonlabs/skeleton';
	import type { Message } from '../types';
	import { parseText } from '$lib/utils';

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
