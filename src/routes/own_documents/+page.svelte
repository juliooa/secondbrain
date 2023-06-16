<script lang="ts">
	import type { PageData } from './$types';
	export let data: PageData;
</script>

<div class="h-full flex flex-col">
	<div class="p-4">
		<h2>Query your documents</h2>
		<div class="flex items-center">
			{#if data.activeModel != null}
				<p class="text-xl text-warning-400">{data.activeModel.name}</p>
			{:else}
				<p class="text-xl text-error-500">No active model</p>
			{/if}
		</div>
	</div>
	<div class="flex justify-center items-center h-full">
		<h3 class="mt-3">Coming soon... 🚀</h3>
	</div>
</div>
