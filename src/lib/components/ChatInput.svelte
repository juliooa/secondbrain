<script lang="ts">
	export let onClearPressed: () => void;
	export let sendMessage: (message: string) => void;
	let inputMessage: string;
</script>

<form
	class="input-group flex flex-row rounded-container-token"
	on:submit|preventDefault={() => {
		sendMessage(inputMessage);
		inputMessage = '';
	}}
>
	<input class="input p-3" placeholder="Enter a message..." bind:value={inputMessage} />
	<button class="btn variant-filled-primary rounded-none" type="submit">
		<iconify-icon icon="ion:paper-plane-outline" /></button
	>
	<button
		class="btn variant-filled-primary rounded-none"
		type="button"
		on:click={() => onClearPressed()}><span>Clear</span></button
	>
</form>
