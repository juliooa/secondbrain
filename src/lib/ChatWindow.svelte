<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { emit, listen } from '@tauri-apps/api/event';
	import { appWindow, WebviewWindow } from '@tauri-apps/api/window';
	import type { Message } from './types';

	let inputMessage = '';
	let messages: Message[] = [];
	let incomingMessage: string = '';

	async function infere() {
		let currentMessage = inputMessage;
		inputMessage = '';
		console.log('infere', currentMessage);
		messages.push({ text: 'Human: ' + currentMessage, role: 'human' } satisfies Message);
		messages.push({ text: '...', role: 'assistant' } satisfies Message);
		messages = messages;
		let answer: string = await invoke('infere', { message: currentMessage });
		console.log(answer);
		messages[messages.length - 1].text = answer;
		messages = messages;
		incomingMessage = '';
	}

	listen('new_token', (event) => {
		console.log(event.event, event.payload);
		incomingMessage = incomingMessage + event.payload.message;
		messages[messages.length - 1].text = incomingMessage;
		messages = messages;
	});

	// emits the `click` event with the object payload
</script>

<div>
	<ul>
		{#each messages as message}
			<li>
				<p>{message.text}</p>
			</li>
		{/each}
	</ul>
	<form on:submit|preventDefault={infere}>
		<input placeholder="Enter a message..." bind:value={inputMessage} />
		<button type="submit">Infere</button>
	</form>
</div>
