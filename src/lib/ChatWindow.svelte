<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { listen } from '@tauri-apps/api/event';
	import type { Message } from './types';
	import 'iconify-icon';
	import MessageBlock from './MessageBlock.svelte';

	let inputMessage = '';
	let messages: Message[] = [];
	let incomingMessage: string = '';

	async function infere() {
		let currentMessage = inputMessage;
		inputMessage = '';
		console.log('infere', currentMessage);
		messages.push({
			text: currentMessage,
			role: 'human',
			id: generateRandomId()
		} satisfies Message);
		messages.push({ text: '...', role: 'assistant', id: generateRandomId() } satisfies Message);
		messages = messages;
		let answer: string = console.log(answer);
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

	// emits the `click` event with the object payload

	function generateRandomId(): number {
		return Math.floor(Math.random() * 100000);
	}
</script>

<div class="flex flex-col h-screen">
	<div class="flex-grow overflow-y-auto">
		{#each messages as message (message.id)}
			<MessageBlock {message} />
		{/each}
	</div>

	<div class="m-3">
		<form
			class="input-group flex flex-row rounded-container-token"
			on:submit|preventDefault={infere}
		>
			<input class="input p-3" placeholder="Enter a message..." bind:value={inputMessage} />
			<button class="btn variant-filled-primary rounded-none" type="submit"
				><span>Send</span>
				<iconify-icon icon="ion:paper-plane-outline" /></button
			>
		</form>
	</div>
</div>
