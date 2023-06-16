<script lang="ts">
	import * as llm from '$lib/llm';
	export let sendMessage: (message: string) => void;
	let inputMessage: string;
	export let isGenerating: boolean = false;

	function stopGenerating() {
		llm.cancelInference();
	}
</script>

<form
	class="input-group flex flex-row rounded-container-token"
	on:submit|preventDefault={() => {
		sendMessage(inputMessage);
		inputMessage = '';
	}}
>
	<input
		class="input p-3"
		placeholder="Enter a message..."
		bind:value={inputMessage}
		disabled={isGenerating}
	/>

	{#if isGenerating}
		<button
			class="btn variant-filled-error rounded-none"
			on:click={() => {
				stopGenerating();
			}}
		>
			Stop
		</button>
	{:else}
		<button class="btn variant-filled-primary rounded-none" type="submit"> Enviar </button>
	{/if}
</form>
