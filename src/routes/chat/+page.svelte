<script lang="ts">
	import type { PageData } from './$types';
	import { listen } from '@tauri-apps/api/event';
	import { MessageRole, type Message, type NewTokenPayload } from '$lib/types';
	import 'iconify-icon';
	import { modalStore, type ModalSettings } from '@skeletonlabs/skeleton';
	import * as llm from '$lib/llm';
	import ChatInput from '$lib/components/ChatInput.svelte';
	import MessageBlock from '$lib/components/MessageBlock.svelte';
	import { generateRandomId } from '$lib/utils';

	export let data: PageData;

	let messages: Message[] = [];
	let incomingMessage: string = '';
	let chatContainer: HTMLElement;
	let isGenerating: boolean = false;

	async function sendMessage(currentMessage: string) {
		messages.push({
			text: currentMessage,
			role: MessageRole.HUMAN,
			id: generateRandomId()
		} satisfies Message);
		messages.push({ text: '...', role: MessageRole.AI, id: generateRandomId() } satisfies Message);
		messages = messages;

		isGenerating = true;
		let answer: string = await llm.chat(currentMessage);
		console.log(answer);
		isGenerating = false;
		messages[messages.length - 1] = {
			text: answer,
			role: MessageRole.AI,
			id: generateRandomId()
		};
		messages = messages;
		incomingMessage = '';
	}

	listen<NewTokenPayload>('new_token', (event) => {
		if (!isGenerating) return;
		console.log(incomingMessage);
		incomingMessage = incomingMessage + event.payload.message;
		console.log(incomingMessage);
		messages[messages.length - 1] = {
			text: incomingMessage,
			role: MessageRole.AI,
			id: generateRandomId()
		};
		messages = messages;
	});

	function onClearPressed(): void {
		const modal: ModalSettings = {
			type: 'confirm',
			title: 'Please Confirm',
			modalClasses: '!bg-red-500',
			buttonTextConfirm: 'Yes, clear it',
			body: 'This will delete all the current conversation. Are you sure?',
			response: (confirm: boolean) => {
				if (confirm) {
					messages = [];
					messages = messages;
				}
			}
		};
		modalStore.trigger(modal);
	}

	function scrollChatBottom(behavior?: ScrollBehavior): void {
		chatContainer.scrollTo({ top: chatContainer.scrollHeight, behavior });
	}
</script>

<div class="flex flex-col h-full">
	<div class="flex flex-row justify-between">
		<div class="p-4">
			<h2>Conversation with</h2>
			<div class="flex items-center">
				{#if data.activeModel != null}
					<p class="text-xl text-warning-400">{data.activeModel.name}</p>
				{:else}
					<p class="text-xl text-error-500">No active model</p>
				{/if}
			</div>
		</div>
		<div class="p-4">
			<button class="btn variant-ringed-primary" type="button" on:click={() => onClearPressed()}>
				<span>Clear conversation</span>
			</button>
		</div>
	</div>
	<div class="flex-grow overflow-y-auto card mx-4" bind:this={chatContainer}>
		{#each messages as message (message.id)}
			<MessageBlock {message} />
		{/each}
	</div>

	<div class="m-3">
		<ChatInput {sendMessage} {isGenerating} />
	</div>
</div>
