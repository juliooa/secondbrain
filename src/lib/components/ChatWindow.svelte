<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import type { Message } from '../types';
	import 'iconify-icon';
	import MessageBlock from './MessageBlock.svelte';
	import { infere } from '../llm';
	import ChatInput from './ChatInput.svelte';

	let messages: Message[] = [];
	let incomingMessage: string = '';
	let chatContainer: HTMLElement;

	async function sendMessage(currentMessage: string) {
		console.log('infere', currentMessage);
		messages.push({
			text: currentMessage,
			role: 'human',
			id: generateRandomId()
		} satisfies Message);
		messages.push({ text: '...', role: 'assistant', id: generateRandomId() } satisfies Message);
		messages = messages;
		let answer: string = await infere(currentMessage);
		console.log(answer);
		messages[messages.length - 1] = {
			text: answer,
			role: 'assistant',
			id: generateRandomId()
		};
		messages = messages;
		incomingMessage = '';
	}

	listen('new_token', (event) => {
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
</script>

<div class="flex flex-col h-screen">
	<div class="flex-grow overflow-y-auto" bind:this={chatContainer}>
		{#each messages as message (message.id)}
			<MessageBlock {message} />
		{/each}
	</div>

	<div class="m-3">
		<ChatInput {onClearPressed} {sendMessage} />
	</div>
</div>
