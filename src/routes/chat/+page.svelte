<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import { MessageRole, type Message, type NewTokenPayload } from '$lib/types';
	import 'iconify-icon';
	import { modalStore, type ModalSettings } from '@skeletonlabs/skeleton';
	import { chat } from '$lib/llm';
	import ChatInput from '$lib/components/ChatInput.svelte';
	import MessageBlock from '$lib/components/MessageBlock.svelte';
	import { getCurrentModel } from '$lib/local_store';
	import { generateRandomId } from '$lib/utils';

	let messages: Message[] = [];
	let incomingMessage: string = '';
	let chatContainer: HTMLElement;

	async function sendMessage(currentMessage: string) {
		messages.push({
			text: currentMessage,
			role: MessageRole.HUMAN,
			id: generateRandomId()
		} satisfies Message);
		messages.push({ text: '...', role: MessageRole.AI, id: generateRandomId() } satisfies Message);
		messages = messages;

		let answer: string = await chat(currentMessage);
		messages[messages.length - 1] = {
			text: answer,
			role: MessageRole.AI,
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
