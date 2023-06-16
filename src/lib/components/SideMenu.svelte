<script lang="ts">
	import { AppRail, AppRailTile, Avatar } from '@skeletonlabs/skeleton';
	import { page } from '$app/stores';
	import { derived } from 'svelte/store';
	import Breathing from './Breathing.svelte';
	const selected = derived(page, ($page) => $page.url.pathname);
</script>

<AppRail {selected} height="h-screen">
	<AppRailTile hover="" href="/">
		<Breathing /></AppRailTile
	>
	<AppRailTile label="Q/A" href="/qa"
		><iconify-icon width="28" icon="mdi:robot-happy-outline" /></AppRailTile
	>
	<AppRailTile label="Chat" href="/chat"
		><iconify-icon width="28" icon="ion:chatbubbles-outline" /></AppRailTile
	>
	<AppRailTile label="Documents" href="/own_documents"
		><iconify-icon width="28" icon="ion:documents-outline" /></AppRailTile
	>
	<svelte:fragment slot="trail">
		<AppRailTile label="Settings" href="/settings"
			><iconify-icon width="28" icon="ion:settings" /></AppRailTile
		>
		<AppRailTile label="Models" href="/models"
			><iconify-icon width="28" icon="ion:settings" /></AppRailTile
		>
	</svelte:fragment>
</AppRail>
<div class="w-full flex flex-col max-h-screen h-screen">
	<div class="flex-grow">
		<slot />
	</div>
</div>
