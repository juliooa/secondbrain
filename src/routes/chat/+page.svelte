<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import type { Message, NewTokenPayload } from '$lib/types';
	import 'iconify-icon';

	import { infere } from '$lib/llm';
	import ChatInput from '$lib/components/ChatInput.svelte';
	import MessageBlock from '$lib/components/MessageBlock.svelte';
	import { getCurrentModelName } from '$lib/local_store';

	let messages: Message[] = [];
	let incomingMessage: string = '';
	let chatContainer: HTMLElement;

	async function sendMessage(currentMessage: string) {
		messages.push({
			text: currentMessage,
			role: 'human',
			id: generateRandomId()
		} satisfies Message);
		messages.push({ text: '...', role: 'assistant', id: generateRandomId() } satisfies Message);
		messages = messages;
		let answer: string = await infere(currentMessage);
		messages[messages.length - 1] = {
			text: answer,
			role: 'assistant',
			id: generateRandomId()
		};
		messages = messages;
		incomingMessage = '';
	}

	listen<NewTokenPayload>('new_token', (event) => {
		console.log(incomingMessage);
		incomingMessage = incomingMessage + event.payload.message;
		console.log(incomingMessage);
		messages[messages.length - 1] = {
			text: incomingMessage,
			role: 'assistant',
			id: generateRandomId()
		};
		messages = messages;
	});

	function generateRandomId(): number {
		return Math.floor(Math.random() * 100000);
	}

	function onClearPressed(): void {
		messages = [];
		messages = messages;
	}

	function scrollChatBottom(behavior?: ScrollBehavior): void {
		chatContainer.scrollTo({ top: chatContainer.scrollHeight, behavior });
	}
	async function getModelName() {
		getCurrentModelName().then((value) => {
			console.log(value);
			modelName = value!;
		});
	}
	getModelName();
	let modelName: string;
</script>

<div class="flex flex-col h-screen">
	<div class="p-4 variant-soft-primary flex flex-row justify-between">
		<h3>
			{#if modelName != null}
				Conversation with <span class="text-xl text-warning-500">{modelName}</span>
			{:else}
				<span class="text-xl text-error-500">No active model</span>
			{/if}
		</h3>
		<button class="btn variant-ringed-primary" type="button" on:click={() => onClearPressed()}
			><span>Clear conversation</span></button
		>
	</div>
	<div class="flex-grow overflow-y-auto" bind:this={chatContainer}>
		{#each messages as message (message.id)}
			<MessageBlock {message} />
		{/each}
	</div>

	<div class="m-3">
		<ChatInput {sendMessage} />
	</div>
</div>
